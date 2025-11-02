//audio_manager.rs
use raylib::prelude::*;

/// Types of background music tracks
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MusicTrack {
    MainMenu,
    Gameplay,
}

/// Types of sound effects
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SoundEffect {
    Shoot,
    RatChase,
    RatAttack,
    RatDeath,
    PlayerHurt,
}

/// Manages all audio in the game
pub struct AudioManager<'a> {
    // Music tracks (with lifetime tied to RaylibAudio)
    menu_music: Music<'a>,
    gameplay_music: Music<'a>,
    current_track: Option<MusicTrack>,

    // Sound effects (with lifetime tied to RaylibAudio)
    shoot_sound: Sound<'a>,
    rat_chase_sound: Sound<'a>,
    rat_attack_sound: Sound<'a>,
    rat_death_sound: Sound<'a>,
    player_hurt_sound: Sound<'a>,

    // Volume controls
    music_volume: f32,
    sfx_volume: f32,

    // Looping sound state (for machine gun)
    is_shoot_looping: bool,
}

impl<'a> AudioManager<'a> {
    /// Volume constants (0.0 to 1.0)
    const MUSIC_VOLUME: f32 = 0.5;
    const SFX_VOLUME: f32 = 0.7;

    /// Create a new audio manager and load all audio files
    /// Requires RaylibAudio handle for loading resources
    pub fn new(audio: &'a RaylibAudio) -> Self {
        // Load music tracks using audio.new_music()
        let mut menu_music = audio
            .new_music("assets/audio/music/menu.mp3")
            .expect("Failed to load menu music");

        let mut gameplay_music = audio
            .new_music("assets/audio/music/gameplay.mp3")
            .expect("Failed to load gameplay music");

        // Load sound effects using audio.new_sound()
        let shoot_sound = audio
            .new_sound("assets/audio/sfx/shoot.mp3")
            .expect("Failed to load shoot sound");

        let rat_chase_sound = audio
            .new_sound("assets/audio/sfx/rat_chase.mp3")
            .expect("Failed to load rat chase sound");

        let rat_attack_sound = audio
            .new_sound("assets/audio/sfx/rat_attack.mp3")
            .expect("Failed to load rat attack sound");

        let rat_death_sound = audio
            .new_sound("assets/audio/sfx/rat_death.mp3")
            .expect("Failed to load rat death sound");

        let player_hurt_sound = audio
            .new_sound("assets/audio/sfx/player_hurt.mp3")
            .expect("Failed to load player hurt sound");

        // Set volumes on the Music objects directly
        menu_music.set_volume(Self::MUSIC_VOLUME);
        gameplay_music.set_volume(Self::MUSIC_VOLUME);

        AudioManager {
            menu_music,
            gameplay_music,
            current_track: None,
            shoot_sound,
            rat_chase_sound,
            rat_attack_sound,
            rat_death_sound,
            player_hurt_sound,
            music_volume: Self::MUSIC_VOLUME,
            sfx_volume: Self::SFX_VOLUME,
            is_shoot_looping: false,
        }
    }

    /// Update audio system (must be called every frame)
    pub fn update(&mut self) {
        // Update current music stream using method on Music object
        if let Some(track) = self.current_track {
            match track {
                MusicTrack::MainMenu => {
                    self.menu_music.update_stream();
                }
                MusicTrack::Gameplay => {
                    self.gameplay_music.update_stream();
                }
            }
        }
    }

    /// Play or switch to a music track
    pub fn play_music(&mut self, track: MusicTrack) {
        // If already playing this track, do nothing
        if self.current_track == Some(track) {
            let is_playing = match track {
                MusicTrack::MainMenu => self.menu_music.is_stream_playing(),
                MusicTrack::Gameplay => self.gameplay_music.is_stream_playing(),
            };

            if is_playing {
                return;
            }
        }

        // Stop current music
        self.stop_music();

        // Play new track using method on Music object
        match track {
            MusicTrack::MainMenu => {
                self.menu_music.play_stream();
            }
            MusicTrack::Gameplay => {
                self.gameplay_music.play_stream();
            }
        }

        self.current_track = Some(track);
    }

    /// Stop current music
    pub fn stop_music(&mut self) {
        if let Some(track) = self.current_track {
            match track {
                MusicTrack::MainMenu => {
                    self.menu_music.stop_stream();
                }
                MusicTrack::Gameplay => {
                    self.gameplay_music.stop_stream();
                }
            }
            self.current_track = None;
        }
    }

    /// Play a one-shot sound effect
    pub fn play_sound(&self, effect: SoundEffect) {
        match effect {
            SoundEffect::Shoot => {
                // Handled by start_shoot_loop/stop_shoot_loop
            }
            SoundEffect::RatChase => {
                self.rat_chase_sound.play();
            }
            SoundEffect::RatAttack => {
                self.rat_attack_sound.play();
            }
            SoundEffect::RatDeath => {
                self.rat_death_sound.play();
            }
            SoundEffect::PlayerHurt => {
                self.player_hurt_sound.play();
            }
        }
    }

    /// Start looping shoot sound (for machine gun)
    pub fn start_shoot_loop(&mut self) {
        if !self.is_shoot_looping {
            self.shoot_sound.play();
            self.is_shoot_looping = true;
        }
    }

    /// Stop looping shoot sound
    pub fn stop_shoot_loop(&mut self) {
        if self.is_shoot_looping {
            self.shoot_sound.stop();
            self.is_shoot_looping = false;
        }
    }

    /// Check if shoot sound is currently looping
    pub fn is_shooting(&self) -> bool {
        self.is_shoot_looping
    }

    /// Set music volume (0.0 to 1.0)
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
        self.menu_music.set_volume(self.music_volume);
        self.gameplay_music.set_volume(self.music_volume);
    }

    /// Set sound effects volume (0.0 to 1.0)
    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
        // Note: Individual sound volume can be set using sound.set_volume()
        // but we'd need to set it before each play, or store it
    }
}
