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
import io.parity.signer.components.BigButton
import io.parity.signer.components.BottomBar
import io.parity.signer.components.TopBar
import io.parity.signer.models.*
import io.parity.signer.screens.LandingView
import io.parity.signer.screens.WaitingScreen
import io.parity.signer.ui.theme.ParitySignerTheme
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.*

@ExperimentalMaterialApi
@ExperimentalAnimationApi
class MainActivity : AppCompatActivity() {
	// rust library is initialized inside data model
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
		val actionResult = signerDataModel.actionResult.observeAsState()
		val shieldAlert = signerDataModel.alertState.observeAsState()
		val progress = signerDataModel.progress.observeAsState()
		val captured = signerDataModel.captured.observeAsState()
		val total = signerDataModel.total.observeAsState()

		when (onBoardingDone.value) {
			OnBoardingState.Yes -> {
				if (authenticated.value == true) {
					BackHandler {
						// TODO: implement this in backend
						if (
							actionResult.value?.alertData == null &&
							actionResult.value?.modalData == null &&
							(
								actionResult.value?.screenData is ScreenData.Log ||
									actionResult.value?.screenData is ScreenData.Scan ||
									actionResult.value?.screenData is ScreenData.SeedSelector ||
									actionResult.value?.screenData is ScreenData.Settings
								)
						) {
							signerDataModel.activity.moveTaskToBack(true)
						} else
							signerDataModel.pushButton(Action.GO_BACK)
					}
					// Structure to contain all app
					Scaffold(
						topBar = {
							TopBar(signerDataModel = signerDataModel, alertState = shieldAlert)
						},
						bottomBar = {
							if (actionResult.value?.footer == true) BottomBar(signerDataModel = signerDataModel)
						}
					) { innerPadding ->
						Box(modifier = Modifier.padding(innerPadding)) {
							ScreenSelector(
								screenData = actionResult.value?.screenData
									?: ScreenData.Documents,//default fallback
								alertState = shieldAlert,
								progress = progress,
								captured = captured,
								total = total,
								seedNames = signerDataModel.seedNames.value ?: emptyArray(),
								isStrongBoxProtected = signerDataModel::isStrongBoxProtected,
								addKey = signerDataModel::addKey,
								checkPath = signerDataModel::checkPath,
								increment = signerDataModel::increment,
								addSeed = signerDataModel::addSeed,
								handleCameraPermissions = signerDataModel::handleCameraPermissions,
								processFrame = signerDataModel::processFrame,
								resetScanValues = signerDataModel::resetScanValues,
								getAppVersion = signerDataModel::getAppVersion,
								wipeToFactory = signerDataModel::wipeToFactory,
								signSufficientCrypto = signerDataModel::signSufficientCrypto,
								signTransaction = signerDataModel::signTransaction,
								wipeToJailbreak = signerDataModel::wipeToJailbreak,
								button = signerDataModel::pushButton

							)
							ModalSelector(
								modalData = actionResult.value?.modalData,
								alertState = shieldAlert,
								removeSeed = signerDataModel::removeSeed,
								getSeedForBackup = signerDataModel::getSeedForBackup,
								addKey = signerDataModel::addKey,
								addSeed = signerDataModel::addSeed,
								selectSeed = signerDataModel::selectSeed,
								button = signerDataModel::pushButton
							)
							AlertSelector(
								alert = actionResult.value?.alertData,
								alertState = shieldAlert,
								button = signerDataModel::pushButton,
								acknowledgeWarning = signerDataModel::acknowledgeWarning
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
							}
						)
						Spacer(Modifier.weight(0.5f))
					}
				}
			}
			OnBoardingState.No -> {
				if (shieldAlert.value == AlertState.None) {
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
							}
						)
						Spacer(Modifier.weight(0.5f))
					}
				}
			}
			null -> WaitingScreen()
		}
	}
}
