package io.parity.signer

import android.os.Bundle
import androidx.activity.compose.BackHandler
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.layout.*
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Scaffold
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.halilibo.richtext.ui.material.SetupMaterialRichText
import io.parity.signer.components.BigButton
import io.parity.signer.components.BottomBar
import io.parity.signer.components.TopBar
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.screens.LandingView
import io.parity.signer.screens.WaitingScreen
import io.parity.signer.ui.theme.ParitySignerTheme
import io.parity.signer.ui.theme.Text600

@ExperimentalMaterialApi
@ExperimentalAnimationApi
class MainActivity : AppCompatActivity() {
	//rust library is initialized inside data model
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
					BackHandler {
						//TODO: implement this in backend
						if (
							signerDataModel.alert.value == SignerAlert.Empty
							&&
							signerDataModel.modal.value == SignerModal.Empty
							&&
							(
								signerDataModel.screen.value == SignerScreen.Log || signerDataModel.screen.value == SignerScreen.Scan || signerDataModel.screen.value == SignerScreen.SeedSelector || signerDataModel.screen.value == SignerScreen.Settings)
						) {
							signerDataModel.activity.moveTaskToBack(true)
						} else
							signerDataModel.pushButton(ButtonID.GoBack)
					}
					//Structure to contain all app
					Scaffold(
						topBar = {
							TopBar(signerDataModel = signerDataModel)
						},
						bottomBar = {
							if (footer.value == true) BottomBar(signerDataModel = signerDataModel)
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
					Scaffold {
						LandingView(signerDataModel::onBoard)
					}
				} else {
					Box(
						contentAlignment = Alignment.Center,
						modifier = Modifier.padding(12.dp)
					) {
						Text(
							"Please enable airplane mode",
							color = MaterialTheme.colors.Text600
						)
					}
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
			null -> WaitingScreen()
		}
	}
}
