Raspberry pi model 3 B pinouts: https://www.raspberrypi.com/documentation/computers/raspberry-pi.html

Deployment steps:
1. Push the changes to the Repo (git push origin master)
2. Pull it on the server (git pull origin master) (legacypi - 252)
3. Build the project and move the binary to `/usr/local/bin/`
4. Update the environment variables if necessary from the Supervisor config
5. Run `reread` and `update` commands on the supervisorctl

Note: `Cross` doesn't work since the build requires some dependecies on openssl which doesn't play well. For now the config is still here.