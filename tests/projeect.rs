use std::fs::OpenOptions;
use std::io::{Write, BufWriter};
use std::io;

use aup3conv::project::Project;
use aup3conv::audacity::audio::AudioLoader;

#[test]
fn read_slice_from_project() {
    let project = Project::new("/home/michael/Music/id-9.aup3");
    let mut a = vec![0f32; 100];
    project.load_audio_slice(0, 100, &mut a);
    assert_ne!(&[0f32], &a[..]);
}
