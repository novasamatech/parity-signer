package io.parity.signer

import android.os.Bundle
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import io.parity.signer.components.BigButton
import io.parity.signer.modals.WaitingScreen
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.ParitySignerTheme
import io.parity.signer.components.BottomBar
import io.parity.signer.components.TopBar
import io.parity.signer.screens.LandingView
import io.parity.signer.ui.theme.Text600

@ExperimentalMaterialApi
@ExperimentalAnimationApi
class MainActivity : AppCompatActivity() {
	private val signerDataModel by viewModels<SignerDataModel>()

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		signerDataModel.context = applicationContext
		signerDataModel.activity = this

		signerDataModel.lateInit()

		setContent {
			SignerApp(signerDataModel)
		}
	}
}

/**
 * Main app component - hosts navhost, Rust-based source of truth, etc.
 */
@ExperimentalMaterialApi
@ExperimentalAnimationApi
@Composable
fun SignerApp(signerDataModel: SignerDataModel) {
	ParitySignerTheme {
		val onBoardingDone = signerDataModel.onBoardingDone.observeAsState()
		val authenticated = signerDataModel.authenticated.observeAsState()
		val signerScreen = signerDataModel.screen.observeAsState()
		val signerModal = signerDataModel.modal.observeAsState()
		val signerAlert = signerDataModel.alert.observeAsState()
		val shieldAlert = signerDataModel.alertState.observeAsState()
		val footer = signerDataModel.footer.observeAsState()

		when (onBoardingDone.value) {
			OnBoardingState.Yes -> {
				if (authenticated.value == true) {
					//Structure to contain all app
					Scaffold(
						topBar = {
							TopBar(signerDataModel = signerDataModel)
						},
						bottomBar = {
							if(footer.value == true) BottomBar(signerDataModel = signerDataModel)
						}
					) { innerPadding ->
						Box(modifier = Modifier.padding(innerPadding)) {
							ScreenSelector(signerScreen.value, signerDataModel)
							ModalSelector(
								modal = signerModal.value ?: SignerModal.Empty,
								signerDataModel = signerDataModel
							)
							AlertSelector(
								alert = signerAlert.value ?: SignerAlert.Empty,
								signerDataModel = signerDataModel
							)
						}
					}
				} else {
					Column(verticalArrangement = Arrangement.Center) {
						Spacer(Modifier.weight(0.5f))
						BigButton(
							text = "Unlock app",
							action = {
								signerDataModel.authentication.authenticate(signerDataModel.activity) {
									signerDataModel.totalRefresh()
								}
							})
						Spacer(Modifier.weight(0.5f))
					}
				}
			}
			OnBoardingState.No -> {
				if (shieldAlert.value == ShieldAlert.None) {
					LandingView(signerDataModel = signerDataModel)
				} else {
					Text("Please enable airplane mode", color = MaterialTheme.colors.Text600)
				}
			}
			OnBoardingState.InProgress -> {
				if (authenticated.value == true) {
					WaitingScreen()
				} else {
					Column(verticalArrangement = Arrangement.Center) {
						Spacer(Modifier.weight(0.5f))
						BigButton(
							text = "Unlock app",
							action = {
								signerDataModel.lateInit()
							})
						Spacer(Modifier.weight(0.5f))
					}
				}
			}
		}
	}
}
