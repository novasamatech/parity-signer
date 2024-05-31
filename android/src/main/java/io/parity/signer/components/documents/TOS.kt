package io.parity.signer.components.documents

import android.content.res.Configuration
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.MarkdownText
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.toRichTextStr
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme

/**
 * Terms and conditions content.
 */

@Composable
fun TosScreen(onBack: Callback) {
	Column(
		Modifier
			.verticalScroll(rememberScrollState())
	) {
		ScreenHeader(
			title = stringResource(R.string.documents_terms_of_service),
			onBack = onBack
		)
		TosText(modifier = Modifier.padding(16.dp))
	}

	BackHandler(onBack = onBack)
}


@Composable
fun TosText(modifier: Modifier = Modifier) {
	MarkdownText(
		modifier = modifier,
		content =
		"""
**General terms and conditions of business and use**

**1. Basic Provisions; Risk Information; No Financial Service**

**1.1** Novasama Technologies GmbH, Schönhauser Allee 163, 10435 Berlin, Germany (hereinafter: “**Novasama**”) offers the mobile application Polkadot Vault (hereinafter: “**Vault**”) to customers as Free-Ware free of charge. The Vault is a non-custodial wallet software that allows the User to turn his/her mobile device, such as smartphone or tablet (hereinafter: “**Storage Device**”), into a cold storage hardware wallet by holding the User’s private cryptographic keys (hereinafter: “**Private Key**”) offline while keeping the Storage Device offline. Novasama does neither store, nor has access to or control over a User's Private Key and seed recovery phrase (hereinafter: “**Mnemonic Phrase**”) at any time.


**1.2** The User can use the Vault to safekeep their Private Keys, manage their accounts and blockchain-based digital assets (hereinafter: “**Digital Assets**”), send and receive transactions of Digital Assets, and cryptographically sign blockchain transactions.

**1.3** By using the Vault, the User confirms that the User is aware of the inherent risks associated with the use of cryptographic and blockchain-based systems and that the User is experienced with securing Private Keys and handling Digital Assets. The User is aware that Digital Asset markets can be highly volatile due to various factors such as speculation, advancing technology, or changes in the regulatory environment and may involve financial risks. Blockchain transactions signed by using the Private Key stored in the Vault and validated by the blockchain network are irrevocable and irreversible, and there is no possibility to retrieve Digital Assets that have been transferred.

**1.4** The User acknowledges that Novasama offers a non-custodial wallet solution. The User is responsible for storing and securing the User’s Mnemonic Phrase and Private Keys, as well as any backups thereof. In particular, the User acknowledges that the failure or loss of the Storage Device can lead to the irreversible loss of all information stored in the Vault including the Mnemonic Phrase and/or Private Key. Therefore, the User commits to keeping a back-up copy of his/her information, including Mnemonic Phrase and Private Key, separate from the Vault and from the Storage Device altogether.

**1.5** Novasama does not store Mnemonic Phrases and Private Keys for the User, nor does it have any backups or access to the User’s Mnemonic Phrase and Private Keys. In the event of loss of the Mnemonic Phrase and Private Keys by the User, Novasama will not be able to recover the User's Private Keys or otherwise restore access to the Vault or any Digital Assets. Novasama is not responsible for the loss of the User’s Mnemonic Phrase or Private Keys. In case of loss, only the User himself can recover his/her Wallet.
**1.6** The User is aware that the use of blockchain infrastructures and the processing of blockchain transactions may result in network fees, which shall be borne by the User. Novasama does not charge the User with these network fees and cannot influence these costs.
**1.7** Novasama does not provide any financial services, in particular Novasama does not give any  investment recommendations or advice and does not provide any trading or crypto custody services. Novasama is also not a provider of Digital Assets, does not offer Digital Assets for sale or purchase. The User is responsible for the tax treatment, in particular income tax treatment, of the profits from the purchase or sale of Digital Assets and use of other services.

**2. Scope of Application; Amendment of the General Terms and Conditions of Business and Use**

**2.1** These General Terms and Conditions of Business and Use (hereinafter: “**GTC**”) apply to contracts concluded between Novasama and the contractual partner (hereinafter: “**User**”) (hereinafter: “**User Agreement**”). The contract is concluded by downloading the Vault on the User’s Storage Device subject to these GTC. The use of the Vault is only permitted to legal entities, partnerships and natural persons with unlimited legal capacity. In particular, minors are prohibited from using the Vault.
‍

**2.2** The application of the User’s general terms and conditions is excluded. Deviating, conflicting or supplementary general terms and conditions of the User shall only become part of the contract if and to the extent that Novasama has expressly agreed to their application in writing. This consent requirement shall apply in any case, even if for example Novasama, being aware of the general terms and conditions of the contractual partner, accepts payments by the contractual partner without reservations.

**2.3** The current version of the GTC is published on Novasama’s website under https://novasama.io, indicating the date of the last update to the GTC. By downloading or updating the Vault, the User accepts the version of the GTC available at the time of his/her latest download or update.

**3. Functions of the Vault; User’s Duty of Care; Third-Party Services.**

**3.1** The Vault allows the User, *inter alia*, to store his/her Private Keys and Mnemonic Phrase offline, to create multiple accounts with derivation paths using his/her Mnemonic Phrase, and to sign transactions or services related to Digital Assets in an air-gapped way without connecting the Storage Device to the internet and/or to other devices. The Vault is native to the Polkadot ecosystem and can be used for any supported blockchain. Novasama continues to develop the Vault on a regular basis, so that the services offered may be subject to change. The presentation of the Vault only reflects the current functionality and does not constitute a binding offer for future services.

**3.2** To use the Vault, the User must choose a Storage Device which the User is willing to permanently disconnect from the internet and from other devices. The User must reset this Storage Device to factory settings, download the Vault and disconnect the Storage Device from the internet and from all other devices. The Vault will then guide the User through the set-up process. The User acknowledges that, once the Vault is set-up, its security benefits as a hardware wallet might be corrupted if the Storage Device is ever connected to the internet and/or to other devices. The User is responsible for keeping the Storage Device safe and offline at all times. Furthermore, the User must frequently update the metadata about blockchain network specifications (hereinafter: “**Metadata**”) stored in the Vault. Otherwise, the User might not be able to, e.g., sign transactions for as long as the Metadata in the Vault is outdated. Up-to-date Metadata is currently provided by Novasama as well as by other providers and can be imported into the Vault in an air-gapped way while keeping the Storage Device offline.

**3.3** From time to time, Novasama may release updated versions of the Vault software. Should the User decide to update the Vault, the User must reset the Storage Device and delete the Vault and the information contained therein prior to connecting the Storage Device to the internet. After that, the User must download and set-up the updated version of the Vault again and add his/her Private Keys, as described in Section 3.2. The User’s obligation to keep back-up copies of his/her information, including Mnemonic Phrase and Private Key, remains unaffected (cf. Section 1.4).

**3.4** The Vault contains certain features (hereinafter: “**Vault Features**”) that, inter alia,  allow the User to connect the Vault to third-party decentralized applications (hereinafter: “**DApp**”), protocols and services that Novasama does not operate itself (hereinafter: “**Third-Party Services**”, cf. Section 4 below). Novasama does not offer Third-Party Services in its own name. The conditions of service provisions for Third-Party Services, if any, shall be governed exclusively by the applicable contractual provisions between the User and the respective provider of the Third-Party Services. Novasama continues to develop the Vault Features on a regular basis, so that the Vault Features offered and Vault's compatibility with Third-Party Services may be subject to change. The presentation of the Vault and the Vault Features in the Vault only reflects the current functionality and does not constitute a binding offer for future services.‍

**4. Conditions of Service; GNU GPLv3**

**4.1** The User's right to use the Vault is limited to the respective state of art. Novasama may temporarily restrict the Vault or certain Vault Features if this is necessary due to capacity limits, to maintain the security or integrity of the software, servers or services or to carry out technical measures, e.g. maintenance work serving the proper or improved service of the Vault. In these cases, Novasama will take the User’s legitimate interests into account, e.g. by providing appropriate information about planned maintenance work in advance. Section 6 of the GTC remains unaffected.

**4.2** To the extent the User connects his/her Vault to Third-Party Services or relies on content or data from third party providers, Novasama does not warrant their accuracy, completeness or timeliness. No contractual relationship exists between Novasama and the Third-Party Service providers and the Third-Party Service providers are not acting as agents of or at the direction of Novasama. Novasama has no technical, legal or organizational means to influence Third-Party Services. Novasama is not liable for any damages incurred by the User due to the use of Third-Party Services, in particular when signing transactions or importing Metadata, such as in case of bridged tokens, unavailability and customization of DApps and protocols, transactions to incompatible, incorrect or unassigned wallet addresses, transactions to DApps, failure of nodes or unavailability of blockchain networks, hacker attacks on DApps, or inaccurate Metadata imported from Third-Party Service providers.
**4.3** Novasama reserves all intellectual property rights in and to the Vault, Vault Features, its software, content, data, or other material. Novasama provides the Vault software code on an "open source" basis in accordance with the terms of the GPL 3.0 license, which can be found here: https://opensource.org/licenses/GPL-3.0, and make the source code of the Vault available here https://github.com/novasamatech. You are free to use, modify and distribute the software code of the Vault in accordance with the terms of GPL 3.0.

**5. Fees**

Blockchain network fees which are due for the execution of transactions shall be borne by the User. Novasama does not charge the User with these network fees and cannot influence these costs. Furthermore, Novasama has no control over the fees charged by the Third-Party Services.

**6. Liability of Novasama; Force Majeure**

**6.1** Except in case of intentional misconduct (*Vorsatz*) or gross negligence (*grobe Fahrlässigkeit*), any liability of Novasama shall be excluded for any and all claims arising in connection to the Vault provided to the Customer as Free-Ware free of charge. Warranty obligations shall only arise if Novasama has fraudulently concealed a possible defect in the Vault or the contractual services.

**6.2** The limitations of liability according to Section 6.1 do not apply (i) concerning damages arising from injury to life, body or health, (ii) as far as Novasama has assumed a guarantee, (iii) to claims of the User according to the Product Liability Act (*Produkthaftungsgesetz*), and (iv) to claims of the User according to any applicable data privacy laws.

**6.3** The liability provisions in Sections 6.1 and 6.2 shall also apply to the personal liability of the executive bodies, legal representatives, employees and vicarious agents of Novasama.

**6.4** If the User suffers damages from the loss of data, Novasama is not liable for this, as far as the damages would have been avoided by a regular and complete backup of all relevant data by the User.

**6.5** Novasama takes all possible measures to enable the User to access the Vault and the Vault Features. In the event of disruptions to the technical infrastructure, the internet connection or a relevant blockchain, Novasama shall be exempt from its obligation to perform. This also applies if Novasama is prevented from performing due to force majeure or other circumstances, the elimination of which is not possible or cannot be economically expected of Novasama.

**6.6** Liability in any other event but Section 6.1 to 6.5 above is excluded.

**7. Indemnities**

**7.1** The User agrees to indemnify Novasama to the extent liable under statutory law from all claims, which other Users or other third parties assert for infringement of their rights against Novasama due to the User’s use of the Vault or other Third-Party Services.

**7.2** In this case, the User assumes all necessary costs of legal defense of Novasama, including all statutory court and attorney fees. This does not apply if the User is not responsible for the infringement.

**7.3** In case of a claim asserted by a third party, the User is obliged to the extent liable under statutory law to provide Novasama with immediate, truthful and complete information necessary for the examination of claims and defense.

**8. Data Protection**

‍Novasama informs the User about Novasama’s processing of personal data, including the disclosure to third parties and the rights of the User as an affected party, in the data protection information.

**9. Final Provisions**

**9.1** Novasama is entitled to transfer its rights and obligations under the User Agreement in whole or in part to third parties with a notice period of four weeks. In this case, the User has the right to terminate the User Agreement without notice.

**9.2** Should individual provisions of these GTC be or become invalid or unenforceable in whole or in part, this shall not affect the validity of the remaining provisions. The invalid or unenforceable provision shall be replaced by the statutory provision. If there is no statutory provision or if the statutory provision would lead to an unacceptable result, the parties shall enter negotiations to replace the invalid or unenforceable provision with a valid provision that comes as close as possible to the economic purpose of the invalid or unenforceable provision.

**9.3** The User Agreement including these GTC shall be governed by German law. The application of the UN Convention on Contracts for the International Sale of Goods is excluded. For consumers domiciled in another European country but Germany, the mandatory provisions of the consumer protection laws of the member state in which the consumer is domiciled shall also apply, provided that these are more advantageous for the consumer than the provisions of the German law.

**9.4** For users who are merchants within the meaning of the German Commercial Code (*Handelsgesetzbuch*), a special fund (*Sondervermögen*) under public law or a legal person under public law, Berlin shall be the exclusive place of jurisdiction for all disputes arising from the contractual relationship.

**9.5** In the event of a conflict between the German and the English version of these GTC, the German version shall prevail.

Last Updated: February 28, 2024
						""".toRichTextStr()
	)
}


@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewTAC() {
	SignerNewTheme {
		//doesn't work in dark mode? Check runtime, it's preview broken for this library
		TosScreen({})
	}
}
