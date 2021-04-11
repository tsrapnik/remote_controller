# instructions ubuntu

First install [elm](https://guide.elm-lang.org/install/elm.html) and [rust](https://www.rust-lang.org/tools/install). For rust you will need to use the nightly version. To do this run following command in the root folder of this project.

```
rustup override set nightly
```

Starting in the root folder of this project, run following commands to build the project.

```
cd client
elm make src/Main.elm --output=script.js
cd ../server
cargo build --release
```

Now we have an executable we give it permissions and run it as a service.

```
cd ..
sudo chmod +x /server/target/release/remote_controller
cd /etc/systemd/system
sudo nano remote_controller.service
```

Write following to the file and save with ctrl+x and y.

```
[Unit]
Description=remote controller

[Service]
Type=simple
Environment=DISPLAY=:0
WorkingDirectory=/home/tsrapnik/projects/remote_controller/pc_server
ExecStart=/home/tsrapnik/projects/remote_controller/pc_server/target/release/remote_controller

[Install]
WantedBy=multi-user.target
```

Of course replace path_to_your_project_folder by the actual path to your project.

We need to give the file certain permissions.

```
sudo chmod 644 /etc/systemd/system/remote_controller.service
```

Now we only need to start the service.

```
sudo systemctl start remote_controller
```

You can check if it is running with following command.

```
sudo systemctl status remote_controller
```

Stop or restart with following.

```
sudo systemctl stop remote_controller
sudo systemctl restart remote_controller
```

If you want to automatically start the service at boot use following.

```
sudo systemctl enable remote_controller
```

When running ubuntu use following command to allow the server through the firewall.

```
sudo ufw allow 80/tcp
```

In stead of running remote_controller servers as root, just let it auto start as currently logged in user. To accomplish this create a `remote_controller.desktop` file in `~/.config/autostart`. Put text below in it.

```
[Desktop Entry]
Type=Application
Path=/home/tsrapnik/projects/remote_controller/pi_server
Exec=/home/tsrapnik/projects/remote_controller/pi_server/target/release/remote_controller
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
Name[en_US]=remote_controller
Name=remote_controller
Comment[en_US]=
Comment=
```

You cannot run remote_controller with a port below 1024 as non root. So just use a port above 1024 and forward all incoming traffic on port 80 to it with command below.

```
sudo iptables -t nat -I PREROUTING -p tcp --dport 80 -j REDIRECT --to-ports 1080
```

Verify with following command.

```
sudo iptables -t nat -L
```

Run commands below to make this change persistent.

```
sudo sh -c "iptables-save > /etc/iptables.rules"
sudo apt-get install iptables-persistent
```

Alternative to autostart on pi is to create /etc/rc.local on the pi and add the following. It runs as root this way.

```
#!/bin/sh -e
#
# rc.local
#
# This script is executed at the end of each multiuser runlevel.
# Make sure that the script will "exit 0" on success or any other
# value on error.
#
# In order to enable or disable this script just change the execution
# bits.
#
# By default this script does nothing.

cd /home/ubuntu/remote_controller/pi_server
/home/ubuntu/remote_controller/pi_server/target/release/remote_controller &

exit 0
```