package io.parity.signer.components

import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import com.halilibo.richtext.markdown.Markdown
import com.halilibo.richtext.ui.RichText
import com.halilibo.richtext.ui.RichTextStyle
import com.halilibo.richtext.ui.material.MaterialRichText
import io.parity.signer.ui.theme.Text600
import io.parity.signer.ui.theme.Typography

/**
 * Terms and conditions content.
 */
@Composable
fun TAC() {
	MaterialRichText() {
		RichText(

		) {
			Markdown(
				"""
**PARITY SIGNER MOBILE APPLICATION - END USER LICENCE AGREEMENT**

**Parity Technologies Limited** is a company registered in England and Wales under company number 09760015, with its registered office at c/o Ignition Law, 1 Sans Walk, London, England, EC1R 0LT ("**Parity**"). Parity operates this Parity Signer mobile application (the "**App**").

In this document, when we refer to "**we**", "**us**" or "**our**", we mean Parity; and when we refer to "**you**" or "**your**" we mean you, the person downloading, accessing and/or using the App.

**1.  Understanding these terms**

  1.1  This end user license agreement together with appendix 1 to it and our instructions referred to in it (this "**EULA**") describes how you may download the App and access and/or use the App. By downloading, accessing and/or using the App, this EULA will apply to you and you agree to the terms of this EULA. You should therefore read the terms carefully before downloading, accessing and/or using the App. If any written instruction we have given you is inconsistent with the terms set out in this document, you should comply with our instructions and in doing so you will not be in breach of the inconsistent term of this document.

  1.2  When certain words and phrases are used in this EULA, they have specific meanings (these are known as "**defined terms**"). You can identify these defined terms because they start with capital letters (even if they are not at the start of a sentence). Where a defined term is used, it has the meaning given to it in the section of this EULA where it was defined (you can find these meanings by looking at the sentence where the defined term is included in brackets and speech marks).

  1.3  In order to access and/or use the App it must be downloaded on to a smartphone or other mobile device running either the iOS or Android operating systems (the device you use, "**Device**"). In order to download the App, you will need Internet access.

  1.4  Please note that:

  1.4.1  to download the App, you must also review and agree to the additional app terms set out in appendix 1 to this EULA and any other terms and conditions imposed by the app store from which you have downloaded the App; and

  1.4.2  we will not collect any personal information from or in relation to you, including from your downloading, accessing and/or use of the App. If you contact us, we only use your personal information in accordance with our privacy policy (available at [*www.parity.io/privacy*](www.parity.io/privacy)).

**2.  The App**

  2.1  Subject to the terms of this EULA, the App is made available to you free of charge. The App does not connect to any computer system (other than the Device on which you install it) and does not gather or transmit to us or any other person any data in relation to you or your use of the App.

  2.2  You are responsible for making all arrangements necessary for you to have access to and use of the App. You should not allow any other person to access and/or use the App on your Device.

  2.3  The App and the content on it are provided for general information purposes only. They are not intended to amount to advice on which you should rely.

  2.4  You may only access and/or use the App for your own domestic, private and non-commercial use.

  2.5  Subject to clause 2.6, the App (once downloaded on to a Device in accordance with this EULA) is designed to enable you to:

  2.5.1  generate private keys using a recovery phrase ("**Private Key(s**)") and a corresponding public key and store them on such Device;

  2.5.2  verify and sign transactions on Blockchains (as defined below) using the relevant Private Key ("**Digital Signature**"); and

  2.5.3  when used in conjunction with certain Third Party Services (which are defined in clause 4.3), can be used to broadcast transactions on the relevant Blockchains,

(the "**App** **Functionality**").

  2.6  The App Functionality the App, can only be used on the Ethereum public blockchain protocol, blockchain protocols built on our "Substrate" technology, including Kusama and Polkadot, and other blockchain protocols as are indicated within the App itself ("**Blockchain(s)**"). You are responsible for your choice of and interaction with the relevant Blockchain.

  2.7  The App is not itself part of any blockchain. For this reason, we will have no liability to you in relation to any activity on any blockchain or the performance or availability of any blockchain. In addition, we do not guarantee that a Digital Signature will result in the related transaction being recorded on the relevant Blockchain. You are responsible for ensuring that any transactions you broadcast, or Digital Signatures you create, conform to the applicable rules of the relevant Blockchain.

  2.8  You should only download, access and/or use the App if you are familiar with interacting with the Blockchains. We recommend you learn about and understand the basics of the Blockchains in connection with which you intend to use the App before downloading, accessing and/or using the App.

  2.9  The purpose of the App is to provide the App Functionality on a Device which is not connected to any form network or the internet. The App should only ever be used on an offline Device. Once you have downloaded, installed and set up the App on the Device, it is very important that you follow the instructions we provide on our website (available at [https://github.com/paritytech/parity-signer](https://github.com/paritytech/parity-signer)) to ensure the Device is not connected to any network or the internet.

  2.10  You are responsible for the use and security of any Device on which you have installed the App. We will have no liability to you for any losses or damages you incur in respect of (i) access and/or use of the App (including the creation of any Digital Signature or loss of Private Keys) as a result of unauthorised persons gaining access to your Device; or (ii) your Device being compromised or corrupted.

**3.  Your account and password**

  3.1  You will need to register an account on the App in order to make full use of the App ("**Account**"). If you register an Account, you will be asked to provide certain information (such as your email address) and to create a pin code and a seed recovery phrase, as part of our security procedures. The information relating to your Account is never transmitted to us. You must treat such pin code and recovery phrase as confidential and you must not disclose it to any third party. Your recovery phrase is used to generate your Private Key, so if you lose your recovery phase, you will not be able to regenerate your Private Key to recover your Account. This may mean you are unable to access any digital currency, token and cryptocurrency associated with such Private Keys and/or create any Digital Signatures. You should ensure that you have taken appropriate steps to securely back-up any of your data stored on your Device (including the recovery phrase and Private Keys) so that such data is accessible in these circumstances.

  3.2  You are responsible for any unauthorised access and/or use of your App or Account login details.

**4.  Acceptable use**

*General*

  4.1  We provide software code in the App on an "open source" basis in accordance with the terms of the GPL 3.0 licence, which can be found here: [*https://opensource.org/licenses/GPL-3.0*](https://opensource.org/licenses/GPL-3.0), ("GPL 3.0") and make the source code of the App available at [*https://github.com/paritytech/parity-signer*](https://github.com/paritytech/parity-signer). You are free to use, modify and distribute the software code in the App in accordance with the terms of GPL 3.0. In the case of any conflict between the terms of GPL 3.0 and the terms of this EULA in relation to your use of the software code in the App, the terms of GPL 3.0 shall prevail.

  4.2  When accessing and/or using the App, you agree:

  4.2.1  not to access and/or use the App in any unlawful manner, for any unlawful purpose or in any manner inconsistent with this EULA;

  4.2.2  not to infringe our intellectual property rights or those of any third party in relation to your use of the App (to the extent that such access and/or use is not licensed under this EULA); and

  4.2.3  to comply with all technology control or export laws and regulations that apply to the technology used or supported by the App.

*Use with other software and hardware*

  4.3  When you make use of the App, you may use the App Functionality to enter into transactions on the relevant Blockchain (for example, transferring crypto currency, entering into a smart contract, or purchasing goods) (“**Transactions**”). You may also use the App in conjunction with other services or software (including cryptographic wallet technology such as software wallets) which are provided by a person or company other than Parity (“**Third Party Services**”). You agree and understand that you are responsible for any Transaction which you enter into and we do not operate or control any such Third Party Services. We have no responsibility for any Transactions you enter into and your use of Third Party Services is at your sole risk and we provide no warranty or guarantee of any in respect of Third Party Services. You further agree that under no circumstances will we be liable to you for any losses or damages arising from any Transactions or any interactions between you and the provider of any such Third Party Services or for any information that such Third Party Services provider provides you or you to them.

*Bugs and Viruses*

  4.4  We do not guarantee that the App will be totally secure or free from bugs or viruses.

  4.5  You must not misuse the App by knowingly introducing viruses, trojans, worms, logic bombs or other material which is malicious or technologically harmful. By breaching this provision, you would commit criminal offences.

**5.  Intellectual property**

  5.1  As we refer to above, we grant to you a licence to use the software code in the App on the terms of GPL 3.0. This right does not apply to the Parity name and mark, the Parity Signer product names, and any texts, designs, graphics, photographs and images contained in the App, which you are not entitled to use (other than through the use of the App on your Device in accordance with this EULA) or modify in any circumstances . We reserve all other rights.

  5.2  We are the owner or licensee of all intellectual property rights in the App and its content, the Parity name and mark and Parity Signer product names and images. Those works are protected by intellectual property laws and treaties around the world. All such rights are reserved.

  5.3  You are not granted any right to use, and may not use, any of our intellectual property rights other than as set out in this EULA. You must not use the App (or any part of it or its content) for commercial purposes.

  5.4  Any communications or materials you send to us by electronic mail or other means will be treated as non-proprietary and non-confidential. We are free to publish, display, post, distribute and otherwise use any ideas, suggestions, concepts, designs, know-how and other information contained in such communications or material for any purpose, including, but not limited to, developing, manufacturing, advertising and marketing us and our products.

**6.  Our liability**

  6.1  Nothing in this EULA excludes or limits our liability for:

  6.1.1  death or personal injury caused by our negligence;

  6.1.2  fraud or fraudulent misrepresentation; and

  6.1.3  any matter in respect of which it would be unlawful for us to exclude or restrict our liability.

  6.2  The App is built on open source software and made available in accordance with this agreement as an open source community project. Accordingly, we make the App (and both the source code and object code in it) available on an "as is" basis. This means that we cannot guarantee or warrant to you that the App is free from errors or defects.

  6.3  It is possible that as a result of an error or defect the version of the App stored on your Device may become corrupt and unusable so that it may not be possible to retrieve the data stored on it (including any Private Keys). You should ensure that you have taken appropriate steps to securely back-up any of your data (including any Private Keys) stored on your Device so that it is not lost in these circumstances. Save if we cause damage to your Device or other digital content and such damage would not have occurred if we had exercised reasonable care and skill, as the App is free for you to access and/or use and we receive no data in relation to you from your use of it, we will not be liable to you in any way if the App stops working properly or at all, including where any data stored in the App is lost or corrupted.

  6.4  Nothing in this EULA affects your statutory rights. Advice about your statutory rights is available from your local Citizens' Advice Bureau or Trading Standards Office.

  6.5  We only supply the App for domestic and private use. You agree not to use the App, or any content on the App, for any commercial or business purposes and we have no liability to you for any loss of profit, loss of business, business interruption, or loss of business opportunity.

  6.6  The App may contain inaccuracies or typographical errors. We make no representations about the reliability, availability, timeliness or accuracy of the content included on the App.

  6.7  We assume no responsibility for the content of any websites or services which you use the App to authenticate transactions on. We will not be liable for any loss or damage that may arise from your use of them.

  6.8  Save as set out in clause 6.1 above, our maximum liability to you under this EULA is £100.

**7.  Suspension and termination**

  7.1  Either you or we may terminate this EULA at any time for any reason.

  7.2  If you breach any of the terms of this EULA, we may immediately do any or all of the following (without limitation):

  7.2.1  issue a warning to you;

  7.2.2  temporarily or permanently withdraw your right to use the App;

  7.2.3  issue legal proceedings against you for reimbursement of all costs resulting from the breach (including, but not limited to, reasonable administrative and legal costs);

  7.2.4  take further legal action against you; and/or

  7.2.5  disclose such information to law enforcement authorities as we reasonably feel is necessary to do so.

  7.3  If we withdraw your right to use the App, then:

  7.3.1  all rights granted to you under this EULA shall cease;

  7.3.2  you must immediately cease all activities authorised by this EULA, including your use of any services provided through the App; and

  7.3.3  you must immediately delete or remove the App from your Device, and immediately destroy all copies of the App then in your possession, custody or control and if we request you to, certify to us that you have done so.

**8.  Changes to this EULA**

  8.1  We may make changes to the terms of this EULA from time to time (if, for example, there is a change in the law that means we need to change this EULA). Please check this EULA (at the address indicated in clause 1.1) regularly to ensure that you understand the up-to-date terms that apply at the time that you access and use the App. If we update the terms of this EULA, the updated terms will apply 10 days after the update is posted (at the address indicated in clause 1.1) to each of your uses of the App from that point on.

  8.2  From time to time updates to the App may be issued through the relevant app store. As the App is designed to operate on a Device which remains offline, you agree that it is your responsibility to check whether we have published any updates to the App. You should carry out this check on a device on which the App has not been installed and follow our instructions (available at [https://github.com/paritytech/parity-signer](https://github.com/paritytech/parity-signer)) as to how to update the App.

  8.3  You will be assumed to have obtained permission from the owners of any Devices that are controlled, but not owned, by you to download a copy of the App onto the Devices. You and they may be charged by your and their service providers for Internet access on the Devices. You accept responsibility in accordance with the terms of this EULA for the use of the App in relation to any Device, whether or not it is owned by you.

**9.  Other important information**

  9.1  Each of the clauses of this EULA operates separately. If any court or relevant authority decides that any of them are unlawful or unenforceable, the remaining clauses will remain in full force and effect.

  9.2  If we fail to insist that you perform any of your obligations under this EULA, or if we do not enforce our rights against you, or if we delay in doing so, that will not mean that we have waived our rights against you and will not mean that you do not have to comply with those obligations. If we do waive a default by you, we will only do so in writing, and that will not mean that we will automatically waive any later default by you.

  9.3  If you wish to have more information on online dispute resolution, please follow this link to the website of the European Commission: [*http://ec.europa.eu/consumers/odr/*](http://ec.europa.eu/consumers/odr/). This link is provided as required by Regulation (EU) No 524/2013 of the European Parliament and of the Council, for information purposes only. We are not obliged to participate in online dispute resolution.

**10.  Governing law and jurisdiction**

  10.1  This EULA is governed by English law. This means that your download, access to, and use of, the App, and any dispute or claim arising out of or in connection therewith will be governed by English law.

  10.2  You can bring proceedings in respect of this EULA in the English courts. However, as a consumer, if you live in another European Union member state you can bring legal proceedings in respect of this EULA in either the English courts or the courts of that Member State.

  10.3  As a consumer, if you are resident in the European Union and we direct this App to the member state in which you are resident, you will benefit from any mandatory provisions of the law of the country in which you are resident. Nothing in this EULA, including clause 10.1, affects your rights as a consumer to rely on such mandatory provisions of local law.

**11.  Contacting us**

Should you have any reasons for a complaint, we will endeavour to resolve the issue and avoid any re-occurrence in the future. You can always contact us by using the following details:
Address: Parity Technologies Limited, c/o Ignition Law, 1 Sans Walk, London, England, EC1R 0LT
Email address: admin@parity.io and legal@parity.io

Thank you.

**Terms last updated 09 Februray 2021**

**APPENDIX 1**

**Additional App Terms**

The following terms and conditions shall apply to your access and/or use of the App in addition to those set out in this EULA.

For the purpose of this appendix 1, "**Appstore Provider**" means the provider of the app store through which you have downloaded the App (for example, Apple is the Appstore Provider if you have downloaded the App from the Apple App Store, Google is the Appstore Provider if you have downloaded the App from Google Play, etc).

1.  You acknowledge and agree that this EULA has been concluded between you and Parity, and not with the Appstore Provider. You acknowledge and agree that the Appstore Provider is not responsible for the App and its content.

2.  You acknowledge and agree that the Appstore Provider has no obligation to provide any maintenance or support in respect of the App. Should you have any problems in using the App, please contact us at admin@parity.io.

3.  In the event that the App does not conform with any product warranty provided for by this EULA, the Appstore Provider may provide you with a refund of the price that you paid to purchase the App (if any). The Appstore Provider shall, to the maximum extent permitted by law, have no obligation to you whatsoever with respect to the App.

4.  You acknowledge and agree that the Appstore Provider shall not be responsible for addressing any claims that you might have relating to the App, including (without limitation): product liability claims; any claim that the App fails to conform to any applicable legal or regulatory requirement; and claims arising under consumer protection or similar legislation.

5.  In the event that a third party claims that the App infringes its intellectual property rights, Parity (and not the Appstore Provider) shall be solely responsible for the investigation, defence, settlement and discharge of such claim.

6.  You warrant and represent that: (i) you are not located in a country that is subject to a U.S. Government embargo, or that has been designated by the U.S. Government as a "terrorist supporting" country; and (ii) you are not listed on any U.S. Government list of prohibited or restricted parties.

If the Appstore Provider is Apple, you acknowledge and agree that Apple and its subsidiaries are third party beneficiaries to this EULA. Upon your acceptance of this EULA, Apple will have the right to enforce the EULA against you as a third party beneficiary.

						"""
			)
		}
	}
}
