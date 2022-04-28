# Upgrading Signer

First of all, you need to be certain you want to upgrade Signer. Starting from v5, all network information could be downloaded through QR codes and upgrades are needed only to add new software features.

## Preparation to upgrade

### Back up your keys

Make sure your keys are backed up - you should have all seed phrases and derivations recorded somewhere. Once you proceed to the next step, there is **no way to recover** lost information.

Make sure to back up all keys for all networks you use. Ideally you should already have some kind of backup, make sure it is up to date.

### Wipe Signer device

Once you are certain that you have backed up everything, open settings, select `Wipe all data` button and confirm your action. All data in the Signer will be factory reset; congratulations!

### Factory reset the phone

When the Signer is removed, wipe the phone to factory state. This is good time to install newer version of operating system if you like. Make sure your system is genuine by all means provided by OS vendor.

### Set up phone

Before installing the Signer, you need to set up the phone. It is essential that you enable sufficient authentication method; your secret seeds in Signer are as safe as the phone is. Seed secrets are protected with hardware encryption based on vendor authentification protocol. Other than that, you might want to select dark mode (Signer remains dark for historic reasons)

### Install Signer

Download signed application through application store or from github. Make sure the signature is valid! Install the app. Do not start the app just yet!

### Disable network

Before starting the Signer, you should make sure that network is disabled. Many operating systems allow only partial network monitoring; although there are network detection features in Signer, they are limited and only have informational function. **User is responsible for maintaining airgapped state!** The simplest way to disable connectivity is setting the phone in airplane mode. Advanced users might want to use physical methods to further protect the phone from connections. Perform all preparations before starting the Signer app!

### Start the Signer

Start the app. Read and accept information provided on launch. Congratulations! The app is ready to use now, however it does not have any keys and only few basic built-in networks. Proceed with [setting up keys](../tutorials/Start.md) and [adding new networks](../tutorials/New-Network.md).
