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
import io.parity.signer.modals.WaitingScreen
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.ParitySignerTheme
import io.parity.signer.components.BottomBar
import io.parity.signer.components.TopBar

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
		val signerScreen = signerDataModel.screen.observeAsState()
		val signerModal = signerDataModel.modal.observeAsState()
		val signerAlert = signerDataModel.alert.observeAsState()

		when (onBoardingDone.value) {
			OnBoardingState.Yes -> {
				//Structure to contain all app
				Scaffold(
					topBar = {
						TopBar(signerDataModel = signerDataModel)
					},
					bottomBar = {
						BottomBar(signerDataModel = signerDataModel)
					}
				) { innerPadding ->
					Box(modifier = Modifier.padding(innerPadding)) {
						ScreenSelector(signerScreen.value, signerDataModel)
						ModalSelector(modal = signerModal.value?: SignerModal.Empty, signerDataModel = signerDataModel)
						AlertSelector(alert = signerAlert.value?: SignerAlert.Empty, signerDataModel = signerDataModel)
					}
				}
			}
			OnBoardingState.No -> {
				//TODO: onboarding
				AlertDialog(
					onDismissRequest = { /*TODO: make sure it is nothing*/ },
					buttons = {
						Button(
							colors = ButtonDefaults.buttonColors(
								backgroundColor = MaterialTheme.colors.background,
								contentColor = MaterialTheme.colors.onBackground,
							),
							onClick = {
								signerDataModel.onBoard()
							}
						) {
							Text("Accept")
						}
					},
					title = { Text("Terms and conditions") },
					text = { Text(onBoardingDone.value.toString()) }
				)
			}
			OnBoardingState.InProgress -> {
				WaitingScreen()
			}
		}
	}
}
