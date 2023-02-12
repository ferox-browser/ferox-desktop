from mlib.command import run_command

def cargo_build(target):
    ctarget = '--release'
    match target:
        case 'release':
            ctarget = '--release'
        case 'debug':
            ctarget = None
            
    print(f"   target {ctarget}")
    
    if ctarget != None:
        run_command("cargo", "build", ctarget)
    else:
        run_command("cargo", "build")