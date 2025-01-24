    // Container lifecycle management
    use incus_client::IncusClient;

    //pub fn start_container(container_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        //let client = IncusClient::new()?;
        //client.start(container_id)?;
        //Ok(())
    //}

    pub fn stop_container(container_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = IncusClient::new()?;
        client.stop(container_id)?;
        Ok(())
    }
