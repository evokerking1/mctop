use std::{collections::HashMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize, ser::SerializeStructVariant};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServerType {
    Vanilla,
    PaperMC,
    Forge,
    Neoforge,
    Fabric,
    Spigot,
}

impl ServerType {
    pub fn as_str(&self) -> &str {
        match self {
            ServerType::Vanilla => "Vanilla",
            ServerType::PaperMC => "PaperMC",
            ServerType::Forge => "Forge",
            ServerType::Neoforge => "NeoForge",
            ServerType::Fabric => "FabricMC",
            ServerType::Spigot => "SpigotMC",
        }
    }

    pub fn variants() -> Vec<ServerType> {
        vec![
            ServerType::Vanilla,
            ServerType::PaperMC,
            ServerType::Forge,
            ServerType::Neoforge,
            ServerType::Fabric,
            ServerType::Spigot,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub id: String,

    pub name: String,

    pub serverType: ServerType,

    pub version: String,

    pub port: u16,

    pub memory_mb: u32,

    path: PathBuf,

    jar_file: String,
}

impl ServerConfig {
    pub fn new(
        name: String,
        server_type: ServerType,
        version: String,
        port: u16,
        memory_mb: u32,
    ) -> Self {
        let id = Uuid::new_v4().to_string();
        let base_path = get_servers_dir();
        let path = base_path.join(&id);

        Self {
            id,
            name,
            server_type,
            version,
            port,
            memory_mb,
            path,
            jar_file: "server.jar".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerStatus {
    Stopped,
    Stopping,
    Starting,
    Running,
}

impl ServerStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ServerStatus::Stopped => "Stopped.",
            ServerStatus::Stopping => "Stopping!!",
            ServerStatus::Starting => "Starting! Please Wait.",
            ServerStatus::Running => "Running, Go ahead and join.",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ServerProperties {
    pub properties: HashMap<String, String>,
}

impl ServerProperties {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let props_path = path.join("server.properties");
        let mut properties = HashMap::new();

        if props_path.exists() {
            let content = fs::read_to_string(&props_path)?;
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((key, value)) = line.split_once('=') {
                    properties.insert(key.trim().to_string(), value.trim().to_string());
                }
            }

            Ok(Self { properties })
        }
    }

    pub fn save(&self, path: PathBuf) -> Result<()> {
        let props_path = .path.join("server.properties")
        let mut content = String::new();
        // Collect The Keys
        let mut keys: Vec<_> = self.properties.keys().collect();
        keys.sort();

        for keys in keys {
            if let Some(Value) = self.properties.get(key) {
                content.push_str(&format!("{}={}\n", key, Value));
            }
        }

        fs::write(props_path, content)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
    }

    pub fn get_common_properties() -> Vec<(&'static str, &'static str)> {
        vec![
            ("server-port", "25565"),
            ("max-players", "20"),
            ("motd", "A Minecraft Server"),
            ("gamemode", "survival"),
            ("difficulty", "easy"),
            ("pvp", "true"),
            ("spawn-protection", "16"),
            ("online-mode", "true"),
            ("white-list", "false"),
            ("enable-command-block", "false"),
            ("spawn-monsters", "true"),
            ("spawn-animals", "true"),
            ("spawn-npcs", "true"),
            ("allow-flight", "false"),
            ("view-distance", "10"),
            ("simulation-distance", "10"),
            ("level-name", "world"),
            ("level-seed", ""),
            ("level-type", "minecraft:normal"),
        ]
    }

    
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpEntry {
    pub uuid: String,
    pub name: String,
    pub level: u8,

    #[serde(rename="bypassesPlayerLimit")]
    pub bypasses_player_limit: bool,
}

pub fn load_ops(server_path: &PathBuf) -> Result<Vec<OpEntry>> {
    let ops_path = server_path.join("ops.json");
    if ops_path.exists() {
        let content = fs::read_to_string(server_path);
        let ops: Vec<OpEntry> = serde_json::from_str(&content)
        Ok(ops)
    } else {
        Ok(Vec::new())
    }
}
