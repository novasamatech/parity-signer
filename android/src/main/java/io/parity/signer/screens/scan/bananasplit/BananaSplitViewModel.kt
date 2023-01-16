package io.parity.signer.screens.scan.bananasplit

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.storage.SeedRepository


class BananaSplitViewModel(): ViewModel() {

	private val uniffiInteractor = ServiceLocator.backendScope.uniffiInteractor
	private val seedRepository: SeedRepository by lazy { ServiceLocator.activityScope!!.seedRepository }



	fun onCancel() {

	}

	fun onDoneTap() {

	}


//	func onCancelTap() {
//		isPresented.toggle()
//	}
//
//	func onDoneTap() {
//		do {
//			let result = try qrparserTryDecodeQrSequence(data: qrCodeData, password: password, cleaned: false)
//				if case let .bBananaSplitRecoveryResult(b: bananaResult) = result,
//				case let .recoveredSeed(s: seedPhrase) = bananaResult {
//					if seedsMediator.checkSeedPhraseCollision(seedPhrase: seedPhrase) {
//						dismissWithError(.seedPhraseAlreadyExists())
//						return
//					}
//					navigation.performFake(navigation: .init(action: .navbarKeys))
//					navigation.performFake(navigation: .init(action: .rightButtonAction))
//					navigation.performFake(navigation: .init(action: .recoverSeed))
//					navigation.performFake(navigation: .init(action: .goForward, details: seedName))
//					seedsMediator.restoreSeed(seedName: seedName, seedPhrase: seedPhrase, navigate: false)
//					navigation.performFake(navigation: .init(action: .goBack))
//					navigation.overrideQRScannerDismissalNavigation = .init(action: .selectSeed, details: seedName)
//					isKeyRecovered = true
//					isPresented.toggle()
//				}
//			} catch QrSequenceDecodeError.BananaSplitWrongPassword {
//				invalidPasswordAttempts += 1
//				if invalidPasswordAttempts > 3 {
//					dismissWithError(.signingForgotPassword())
//					return
//				}
//				isPasswordValid = false
//			} catch let QrSequenceDecodeError.BananaSplit(s: errorDetail) {
//				dismissWithError(.alertError(message: errorDetail))
//			} catch let QrSequenceDecodeError.GenericError(s: errorDetail) {
//				dismissWithError(.alertError(message: errorDetail))
//			} catch {
//				dismissWithError(.alertError(message: error.localizedDescription))
//			}
//		}
}
