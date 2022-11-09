package io.parity.signer

import android.graphics.Color
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
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.core.view.WindowCompat
import io.parity.signer.components.BigButton
import io.parity.signer.components.BottomBar
import io.parity.signer.components.panels.TopBar
import io.parity.signer.models.AlertState
import io.parity.signer.models.NavigationMigrations
import io.parity.signer.models.SignerDataModel
import io.parity.signer.screens.LandingView
import io.parity.signer.screens.WaitingScreen
import io.parity.signer.ui.navigationselectors.*
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.ScreenData
import io.parity.signer.uniffi.initLogging

@ExperimentalMaterialApi
@ExperimentalAnimationApi
class MainActivity : AppCompatActivity() {
	init {
		initLogging("SIGNER_RUST_LOG")
	}

	// rust library is initialized inside data model
	private val signerDataModel by viewModels<SignerDataModel>()

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		signerDataModel.context = applicationContext
		signerDataModel.activity = this

		signerDataModel.lateInit()

		//remove automatic insets so bottom sheet can dimm status bar, other views will add their paddings if needed.
		WindowCompat.setDecorFitsSystemWindows(window, false)
		window.statusBarColor = Color.TRANSPARENT;

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
	SignerNewTheme {
		val onBoardingDone = signerDataModel.onBoardingDone.observeAsState()
		val authenticated = signerDataModel.authenticated.observeAsState()
		val actionResult = signerDataModel.actionResult.observeAsState()
		val shieldAlert = signerDataModel.alertState.observeAsState()
		val localNavAction = signerDataModel.localNavAction.observeAsState()

		when (onBoardingDone.value) {
			OnboardingWasShown.Yes -> {
				if (authenticated.value == true) {
					BackHandler {
						signerDataModel.navigator.backAction()
					}
					// Structure to contain all app
					Box {
						//screens before redesign
						Scaffold(
							modifier = Modifier
								.navigationBarsPadding()
								.captionBarPadding()
								.statusBarsPadding(),
							topBar = {
								if (NavigationMigrations.shouldShowBar(
										localNavAction = localNavAction.value,
										globalNavAction = actionResult.value,
									)
								) {
									TopBar(
										signerDataModel = signerDataModel,
										alertState = shieldAlert,
									)
								}
							},
							bottomBar = {
								if (NavigationMigrations.shouldShowBar(
										localNavAction = localNavAction.value,
										globalNavAction = actionResult.value,
									)
									&& actionResult.value?.footer == true
								) {
									BottomBar(signerDataModel = signerDataModel)
								}
							},
						) { innerPadding ->
							Box(modifier = Modifier.padding(innerPadding)) {
								ScreenSelector(
									screenData = actionResult.value?.screenData
										?: ScreenData.Documents,//default fallback
									alertState = shieldAlert,
									button = signerDataModel.navigator::navigate,
									signerDataModel = signerDataModel
								)
								ModalSelector(
									modalData = actionResult.value?.modalData,
									localNavAction = localNavAction.value,
									alertState = shieldAlert,
									button = signerDataModel.navigator::navigate,
									signerDataModel = signerDataModel,
								)
								AlertSelector(
									alert = actionResult.value?.alertData,
									alertState = shieldAlert,
									button = signerDataModel.navigator::navigate,
									acknowledgeWarning = signerDataModel::acknowledgeWarning
								)
							}
							//new screens selectors
							Box(
								modifier = Modifier
									.navigationBarsPadding()
									.captionBarPadding(),
							) {
								CombinedScreensSelector(
									screenData = actionResult.value?.screenData
										?: ScreenData.Documents,//default fallback
									alertState = shieldAlert,
									signerDataModel = signerDataModel
								)
								BottomSheetSelector(
									modalData = actionResult.value?.modalData,
									localNavAction = localNavAction.value,
									alertState = shieldAlert,
									signerDataModel = signerDataModel,
									navigator = signerDataModel.navigator,
								)
							}
						}
					}
				} else {
					Column(verticalArrangement = Arrangement.Center) {
						Spacer(Modifier.weight(0.5f))
						BigButton(
							text = stringResource(R.string.unlock_app_button),
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
			OnboardingWasShown.No -> {
				if (shieldAlert.value == AlertState.None) {
					Scaffold(
						modifier = Modifier
							.navigationBarsPadding()
							.captionBarPadding()
							.statusBarsPadding(),
					) { padding ->
						LandingView(
							signerDataModel::onBoard,
							modifier = Modifier.padding(padding)
						)
					}
				} else {
					Box(
						contentAlignment = Alignment.Center,
						modifier = Modifier.padding(12.dp),
					) {
						Text(
							"Please enable airplane mode",
							color = MaterialTheme.colors.Text600
						)
					}
				}
			}
			OnboardingWasShown.InProgress -> {
				if (authenticated.value == true) {
					WaitingScreen()
				} else {
					Column(verticalArrangement = Arrangement.Center) {
						Spacer(Modifier.weight(0.5f))
						BigButton(
							text = stringResource(R.string.unlock_app_button),
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
