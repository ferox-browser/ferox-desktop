from mlib.command import run_command

def cargo_build(target):
    ctarget = '--release'
    match ctarget:
        case 'release':
            target = '--release'
        case 'dev'|'debug':
            target = ''
    
    run_command("cargo", "build", ctarget)