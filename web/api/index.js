var spawn = require('child_process').fork;

spawn('cargo', ['run', '--manifest-path', 'api/Cargo.toml'], { stdio: 'inherit' });
