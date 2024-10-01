pub mod player;
use player::Player;

// https://hub.spigotmc.org/javadocs/spigot/org/bukkit/entity/Entity.html

pub enum EntityType {
    Player(Player),
}
