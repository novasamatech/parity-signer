## FAQ

- **What is Signer?**
    
    Signer is an app for an air-gapped device, it turns an offline device — usually a smartphone — into a secure hardware wallet. Signer offers you a way to securely generate, store, manage and use your blockchain credentials. 
    
- **How does an offline device communicate with the outside world?**
    
    Communication happens through scanning and generating QR codes. Scanned with Signer input-QRs interact with keys stored in Signer to, generate response-QRs on behalf of those keys. Usually input-QR is a blockchain transaction, and a response-QR is a signature for this transaction. There are tried and true cryptographic algorithms that power these QR codes, as well as some smart engineering that make your dedicated device safe to use.
    
- **Should I use Signer?**
    
    Signer is optimised for the highest security requirements. If you already manage many accounts on multiple networks, Signer is great for you. If you have little experience with blockchain networks but still want good security affordances, you might find the learning curve steep. We strive to make Signer as intuitive as possible; get in touch via [signer@parity.io](mailto:signer@parity.io) or [GitHub Issues](https://github.com/paritytech/parity-signer/issues) if you can help us get there!
    
- **What networks does Signer support?**
    
    From-the-shelf Party Signer supports Polkadot, Kusama and Westend networks. But it's not limited to these networks. More experienced users can generate metadata to expand network capability of Parity Signer. 
    
- **I want to play with Signer to get a better feeling of how it works. Is there a way to do it without spending valuable tokens?**
    
    Yes. In Signer, you should add a key for an address on Westend network and request test tokens for that address, see step-by-step guide on [Polkadot Network Wiki](https://wiki.polkadot.network/docs/learn-DOT#getting-westies). 
    
    You can use test tokens in the same way you would use value-bearing tokens.
    
    For example with [PolkadotJS Apps](https://polkadot.js.org/apps/) you can create a transaction on behalf of your account, generate a signature with Signer and submit it to the network. All of this without keys ever leaving your offline device.
    
- **How do I keep my keys secure?**
    
    Signer is a safe way to use your keys. However, that alone won't be enough to keep your keys secure. Devices break and get lost. This is why we always recommend to backup your seed phrases and derivation paths on paper. We are such big fans of paper backups that we even support a special tool to power your paper backup game by splitting your backups into shards called [Banana Split](https://bs.parity.io/).
    
- **How do I know I am not interacting with malicious apps or actors?**
    
    Signer is does not interact with network. The app itself does not have a way to check if an app or an account you interacting with is malicious. 
    If you use Signer with PolkadotJS Browser Extension, PolkadotJS Apps or Signer Component Browser Extension they will rely on community driven curated list of potentially less-than-honest operators: [https://polkadot.js.org/phishing/#](https://polkadot.js.org/phishing/#) to prevent you from interacting with certain sites and addresses. However, there are no limitations on use of Signer with other tools.
	
- **Can import my account from polkadot{.js} apps or extension to Parity Signer?**

	Yes. Keys are compatible between polkadot{.js} and Parity Signer, except for Ledger keys. To import seed keys into Parity Signer, you need to know:
1. Seed phrase\
_It should always be backed up in paper!_
2. Network you are adding address to and whether Parity Signer installed on your device has metadata for the respective network.\
_If (2) is not one of the default built-in networks, you will need to add network yourself or find a distribution center for adding networks._
3. Derivation path\
_Only if you are importing a derived key, usually keys generated with polkadot{.js} are seed keys._

	In Parity Signer go to Keys, then press "Plus" icon in the top right of the screen, select "Recover seed", enter display name to identify your seed, press "Next", enter the seed phrase. Done, you've got your seed key imported!\
	If you are importing a derived key select the seed from which your key is derived, select account's network, press "Plus" icon next to "Derived keys", enter your derivation path.
    
- **What is a difference between seed key and derived key? Why should I use derived keys?**
    
    A seed key is a single key pair generated from a seed phrase. You can “grow” as many derived keys from a single seed by adding derivation paths to your seed phrase.
    
    Learn more about types of derivation paths on [substrate.io](https://docs.substrate.io/v3/tools/subkey/#hd-key-derivation).
    
    Derivation path is a sensitive information, but knowing the derivation path is not enough to recover a key. Derived keys cannot be backed up without both of the ingredients: seed phrase (can be shared between multiple keys) and a derivation path (unique for each of the keys “grown” from that seed).
    
    The main reason to use derived keys is how easy it is to backup (and restore from a backup) a derivation path compared to seed phrase.
    
- **What is an identicon, the image next to my keys?**
    
    An identicon is a visual hash of a public key — a unique picture generated from your public key. The same public key should have the same identicon regardless of the application. It is a good tool to distinguish quickly between keys. However, when interacting with keys, i.g. verifying a recipient of a transaction, do not rely only on identicons, it is better to check the full public address.
    
- **Signer does not decode transactions when I scan them. What happened? How can I fix it?**
    
    Most likely network you are interacting with was updated and Signer is missing data to decode the transaction. You should update network’s metadata by scanning a QR code containing recent metadata – the same way you scan a transaction QR code – on [Metadata Update Portal](https://metadata.parity.io/).
    
- **How can I rename one of my seeds?**
    
    Due to security considerations, you cannot rename a seed. Please backup the seed and derived keys, remove it and add the seed again with a new name instead.
