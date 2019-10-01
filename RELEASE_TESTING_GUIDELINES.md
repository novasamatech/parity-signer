### Unit Testing
Basically any method in `src/util` should be unit tested via Jest.

### E2E Testing (Ethereum Legacy)

1. Recover an account on Kovan with the phrase "this is sparta"
2. Go to MyCrypto and generate a transaction
3. Scan transaction QR
4. Scan the signature back to MyCrypto
5. Make sure the transaction actually succeeds
6. Go to Sign a Message tab in MyCrypto now
7. Type in a message, any message
8. Repeat steps 3, 4
9. Expect the message is readable just as you typed it in
10. Scan the signature back to MyCrypto
11. Verify the message

### E2E Testing (UOS)
There is currently no testnet so we just check that the signature is formatted correctly. 

1. Clone  https://github.com/polkadot-js/apps
2. Disable the balance validation in apps so you can construct an extrinsic with 0 balance
3. Choose Kusama network in Settings Tab
4. Do any extrinsic of your choice
5. Scan QR
5. Go to Settings, and change to Substrate (either the hosted Flaming Fir or a custom chain)
6. Run the same transaction
7. Expect the message: "Signer does not currently support a chain with the genesis hash: 0x....."
8. Go to `constants.js`
9. Add the appropriate genesis hash
10. Uncomment the checks for `isSubstrateDev` in `PayloadDetailsCard.js`
11. Repeat steps 4, 5
12. Expect the method decoding to be raw bytes.