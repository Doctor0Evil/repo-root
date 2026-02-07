use tracing::info;

pub struct VmDeployer {
    vm_image: String,
}

impl VmDeployer {
    pub fn new(vm_image: String) -> Self {
        Self { vm_image }
    }

    pub async fn deploy_vm(
        &self,
        location: &str,
        deploy_contract: &bool,
        lan_mode: &str,
    ) -> anyhow::Result<()> {
        info!(
            "Deploying ALN VM image {} to {} with contract={}, lan_mode={}",
            self.vm_image, location, deploy_contract, lan_mode
        );
        Ok(())
    }
}
