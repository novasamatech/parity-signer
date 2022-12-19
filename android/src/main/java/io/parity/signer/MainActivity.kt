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
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.core.view.WindowCompat
import io.parity.signer.components.BigButton
import io.parity.signer.components.BottomBar
import io.parity.signer.components.panels.TopBar
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.AlertState
import io.parity.signer.models.NavigationMigrations
import io.parity.signer.models.SignerDataModel
import io.parity.signer.screens.LandingView
import io.parity.signer.screens.WaitingScreen
import io.parity.signer.ui.navigationselectors.*
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.ScreenData

@ExperimentalMaterialApi
@ExperimentalAnimationApi
class MainActivity : AppCompatActivity() {

	// rust library is initialized inside data model
	private val signerDataModel by viewModels<SignerDataModel>()

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		ServiceLocator.initActivityDependencies(this)

		if (savedInstanceState == null) {
			signerDataModel.lateInit()
		}

		//remove automatic insets so bottom sheet can dimm status bar, other views will add their paddings if needed.
		WindowCompat.setDecorFitsSystemWindows(window, false)
		window.statusBarColor = Color.TRANSPARENT;

		setContent {
			SignerApp(signerDataModel)
		}
	}

	override fun onDestroy() {
		ServiceLocator.deinitActivityDependencies()
		super.onDestroy()
	}
}


@ExperimentalMaterialApi
@ExperimentalAnimationApi
@Composable
fun SignerApp(signerDataModel: SignerDataModel) {
	SignerNewTheme {
		val onBoardingDone = signerDataModel.onBoardingDone.collectAsState()
		val authenticated = signerDataModel.authenticated.collectAsState()
		val actionResult = signerDataModel.actionResult.collectAsState()
		val shieldAlert = signerDataModel.alertState.collectAsState()
		val localNavAction = signerDataModel.localNavAction.collectAsState()

		when (onBoardingDone.value) {
			OnboardingWasShown.Yes -> {
				if (authenticated.value) {
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
										globalNavAction = actionResult.value,)
									&& actionResult.value.footer
								) {
									BottomBar(signerDataModel = signerDataModel)
								}
							},
						) { innerPadding ->
							Box(modifier = Modifier.padding(innerPadding)) {
								ScreenSelector(
									screenData = actionResult.value.screenData,
									alertState = shieldAlert,
									navigate = signerDataModel.navigator::navigate,
									signerDataModel = signerDataModel
								)
								ModalSelector(
									modalData = actionResult.value.modalData,
									localNavAction = localNavAction.value,
									alertState = shieldAlert,
									navigate = signerDataModel.navigator::navigate,
									signerDataModel = signerDataModel,
								)
							}
						}
						//new screens selectors
						Box(
							modifier = Modifier
								.navigationBarsPadding()
								.captionBarPadding(),
						) {
							CombinedScreensSelector(
								screenData = actionResult.value.screenData,
								localNavAction = localNavAction.value,
								alertState = shieldAlert,
								signerDataModel = signerDataModel
							)
							BottomSheetSelector(
								modalData = actionResult.value.modalData,
								localNavAction = localNavAction.value,
								alertState = shieldAlert,
								signerDataModel = signerDataModel,
								navigator = signerDataModel.navigator,
							)
							AlertSelector(
								alert = actionResult.value.alertData,
								alertState = shieldAlert,
								navigate = signerDataModel.navigator::navigate,
								acknowledgeWarning = signerDataModel::acknowledgeWarning
							)
						}
					}
				} else {
					Column(verticalArrangement = Arrangement.Center) {
						Spacer(Modifier.weight(0.5f))
						BigButton(
							text = stringResource(R.string.unlock_app_button),
							action = {
								ServiceLocator.authentication.authenticate(signerDataModel.activity) {
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
						modifier = Modifier.padding(12.dp).fillMaxSize(1f),
					) {
						Text(
							text = stringResource(R.string.enable_airplane_mode_error),
							color = MaterialTheme.colors.Text600
						)
					}
				}
			}
			OnboardingWasShown.InProgress -> {
				if (authenticated.value) {
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
		}
	}
}
