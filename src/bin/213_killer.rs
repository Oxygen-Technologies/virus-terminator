use std::env;
use std::process::Command;
use std::path::{PathBuf};

fn main() {
    let maya_location_path = PathBuf::from(env::args().nth(1).expect("Need Maya Location Path."));
    let maya_file_path = PathBuf::from(env::args().nth(2).expect("Need Maya File Path."));

    let mata_file_type = if maya_file_path.extension().unwrap() == "mb" {
        "mayaBinary"
    } else if maya_file_path.extension().unwrap() == "ma" {
        "mayaAscii"
    } else {
        panic!("Maya File Error.")
    };

    let mayapy_exec_path = PathBuf::from(format!("{}/bin/mayapy.exe", maya_location_path.display()));

    let virus_213_killer_py_path = format!(r#"{}\vendors\virus_213_killer_py"#, env::current_dir().unwrap().display());

    let py_script = format!(r#"import sys;sys.path.append(r'{virus_213_killer_py_path}');import maya.standalone as standalone;standalone.initialize();import maya.cmds as cmds;from virus_213_killer_py import kill, save_anim_mel_file;cmds.file(r'{maya_file_path}', open=True);kill();save_anim_mel_file(r'{maya_location}');cmds.file(force=True, type='{maya_file_type}', save=True);print 'Done'"#,
                            virus_213_killer_py_path = virus_213_killer_py_path,
                            maya_file_path = maya_file_path.display(),
                            maya_file_type = mata_file_type,
                            maya_location = maya_location_path.display(),
    );

    let output = Command::new(mayapy_exec_path).arg("-c").arg(py_script).output().expect("MayaPy Error.");
    println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
    println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
}