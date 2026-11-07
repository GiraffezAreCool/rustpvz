use anyhow::Result;
use ash::vk;
use std::collections::HashSet;
use std::sync::Arc;
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

pub struct RenderingContext {
    pub queues: Vec<vk::Queue>,
    pub device: ash::Device,
    pub queue_family_indices: HashSet<u32>,
    pub queue_families: QueueFamilies,
    pub physical_device: PhysicalDevice,
    pub surface_extention: ash::khr::surface::Instance,
    pub instance: ash::Instance,
    pub entry: ash::Entry,
}

#[derive(Debug, Clone)]
pub struct QueueFamily {
    pub index: u32,
    pub queue_family_properties: vk::QueueFamilyProperties,
}
#[derive(Debug)]
pub struct PhysicalDevice {
    pub handle: vk::PhysicalDevice,
    pub properties: vk::PhysicalDeviceProperties,
    pub features: vk::PhysicalDeviceFeatures,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub queue_families: Vec<QueueFamily>,
}

type QueueFamilyPicker = fn(Vec<PhysicalDevice>) -> Result<(PhysicalDevice, QueueFamilies)>;

pub struct RenderingContextAttributes<'window> {
    pub compatibility_window: &'window Window,
    pub queue_family_picker: QueueFamilyPicker,
}

pub struct QueueFamilies {
    pub graphics: u32,
    pub present: u32,
    pub transfer: u32,
    pub compute: u32,
}

pub mod queue_family_picker {
    use crate::app::engine::rendering_context::{PhysicalDevice, QueueFamilies};
    use anyhow::Context as AnyhowContext;
    use anyhow::Result;
    use ash::vk;
    pub fn single_queue_family(
        physical_devices: Vec<PhysicalDevice>,
    ) -> Result<(PhysicalDevice, QueueFamilies)> {
        let physical_device = physical_devices.into_iter().next().unwrap();
        let queue_family = physical_device
            .queue_families
            .iter()
            .find(|queue_family| {
                queue_family
                    .queue_family_properties
                    .queue_flags
                    .contains(vk::QueueFlags::GRAPHICS)
                    && queue_family
                        .queue_family_properties
                        .queue_flags
                        .contains(vk::QueueFlags::COMPUTE)
            })
            .map(|queue_family| queue_family.index)
            .context("No suitable queue family found")?;
        Ok((
            physical_device,
            QueueFamilies {
                graphics: queue_family,
                present: queue_family,
                transfer: queue_family,
                compute: queue_family,
            },
        ))
    }
}

impl RenderingContext {
    pub fn new(attributes: RenderingContextAttributes) -> Result<Self> {
        unsafe {
            let entry = ash::Entry::load()?;
            let raw_display_handle = attributes.compatibility_window.display_handle()?.as_raw();
            let raw_window_handle = attributes.compatibility_window.window_handle()?.as_raw();
            let instance = entry.create_instance(
                &vk::InstanceCreateInfo::default()
                    .application_info(
                        &vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_3),
                    )
                    .enabled_extension_names(ash_window::enumerate_required_extensions(
                        raw_display_handle,
                    )?),
                None,
            )?;

            let surface_extention = ash::khr::surface::Instance::new(&entry, &instance);

            let compatibility_surface = ash_window::create_surface(
                &entry,
                &instance,
                raw_display_handle,
                raw_window_handle,
                None,
            )?;

            let mut physical_devices = instance
                .enumerate_physical_devices()?
                .into_iter()
                .map(|handle| {
                    let properties = instance.get_physical_device_properties(handle);
                    let features = instance.get_physical_device_features(handle);
                    let memory_properties = instance.get_physical_device_memory_properties(handle);
                    let queue_family_properties =
                        instance.get_physical_device_queue_family_properties(handle);

                    let queue_families = queue_family_properties
                        .into_iter()
                        .enumerate()
                        .map(|(index, properties)| QueueFamily {
                            index: index as u32,
                            queue_family_properties: properties,
                        })
                        .collect::<Vec<_>>();

                    PhysicalDevice {
                        handle,
                        properties,
                        features,
                        memory_properties,
                        queue_families,
                    }
                })
                .collect::<Vec<_>>();

            //println!("{:#?}", physical_devices);

            physical_devices.retain(|device| {
                surface_extention
                    .get_physical_device_surface_support(device.handle, 0, compatibility_surface)
                    .unwrap_or(false)
            });

            surface_extention.destroy_surface(compatibility_surface, None);

            let (physical_device, queue_families) =
                (attributes.queue_family_picker)(physical_devices)?;

            let queue_family_indices = HashSet::from([
                queue_families.graphics,
                queue_families.present,
                queue_families.transfer,
                queue_families.compute,
            ]);

            let queue_create_infos = queue_family_indices
                .iter()
                .copied()
                .map(|index| {
                    vk::DeviceQueueCreateInfo::default()
                        .queue_family_index(index)
                        .queue_priorities(&[1.0])
                })
                .collect::<Vec<_>>();

            let device = instance.create_device(
                physical_device.handle,
                &vk::DeviceCreateInfo::default()
                    .queue_create_infos(&queue_create_infos)
                    .enabled_extension_names(&[ash::khr::swapchain::NAME.as_ptr()])
                    .push_next(
                        &mut vk::PhysicalDeviceDynamicRenderingFeatures::default()
                            .dynamic_rendering(true),
                    )
                    .push_next(
                        &mut vk::PhysicalDeviceBufferDeviceAddressFeatures::default()
                            .buffer_device_address(true),
                    ),
                None,
            )?;

            let queues = queue_family_indices
                .iter()
                .map(|index| device.get_device_queue(*index, 0))
                .collect::<Vec<_>>();

            Ok(Self {
                queues,
                device,
                queue_family_indices,
                queue_families,
                physical_device,
                surface_extention,
                instance,
                entry,
            })
        }
    }
}

impl Drop for RenderingContext {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}
