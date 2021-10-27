package io.parity.signer

import android.os.Bundle
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.Lock
import androidx.compose.material.icons.filled.Settings
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.core.content.ContextCompat
import io.parity.signer.modals.WaitingScreen
import io.parity.signer.models.SignerDataModel
import io.parity.signer.screens.HomeScreen
import io.parity.signer.screens.KeyManager
import io.parity.signer.screens.SettingsScreen
import io.parity.signer.ui.theme.ParitySignerTheme
import android.Manifest
import android.content.pm.PackageManager
import androidx.core.app.ActivityCompat
import io.parity.signer.components.BottomBar
import io.parity.signer.components.TopBar
import io.parity.signer.screens.HistoryScreen

class MainActivity : AppCompatActivity() {
	private val REQUIRED_PERMISSIONS = arrayOf(Manifest.permission.CAMERA)
	private val REQUEST_CODE_PERMISSIONS = 10

	private val signerDataModel by viewModels<SignerDataModel>()

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		signerDataModel.context = applicationContext
		signerDataModel.activity = this

		//TODO: testing to make sure this goes smoothly
		if (!allPermissionsGranted()) {
			ActivityCompat.requestPermissions(
				this,
				REQUIRED_PERMISSIONS,
				REQUEST_CODE_PERMISSIONS
			)
		}

		signerDataModel.lateInit()
		setContent {
			SignerApp(signerDataModel)
		}
	}

	private fun allPermissionsGranted() = REQUIRED_PERMISSIONS.all {
		ContextCompat.checkSelfPermission(
			baseContext, it
		) == PackageManager.PERMISSION_GRANTED
	}


}

/**
 * Main app component - hosts navhost, Rust-based source of truth, etc.
 */
@Composable
fun SignerApp(signerDataModel: SignerDataModel) {
	ParitySignerTheme {
		//val allScreens = SignerScreen.values().toList()
		var onBoardingDone = signerDataModel.onBoardingDone.observeAsState()
		var signerScreen = signerDataModel.signerScreen.observeAsState()

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
				) { _ ->
					when (signerScreen.value) {
						SignerScreen.Scan -> {
							HomeScreen(
								signerDataModel = signerDataModel)
						}
						SignerScreen.Keys -> {
							KeyManager(signerDataModel = signerDataModel)
						}
						SignerScreen.Settings -> {
							SettingsScreen(signerDataModel = signerDataModel)
						}
						SignerScreen.Log -> {
							HistoryScreen(signerDataModel = signerDataModel)
						}
					}
				}
			}
			OnBoardingState.No -> {
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

		/*
		Surface(color = MaterialTheme.colors.background) {
			MainScreen(signerDataModel)
		}
		{
				TopBar(SignerScreen.Home, navBack = { navController.navigateUp() })
			},
			BottomBar(
					signerDataModel = signerDataModel,
					navToKeys = { navController.navigate(SignerScreen.Keys.name) },
					navToSettings = { navController.navigate(SignerScreen.Settings.name) })
		*/
	}
}

/**
 * TODO: remove junk
 */

/*
@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
	val signerDataModel by viewModels<SignerDataModel>()
	ParitySignerTheme {
				MainScreen(signerDataModel)
    }
}

val navState: NavState by signerDataModel.navState.observeAsState(NavState.home)
		when (signerDataModel.navState.value) {
				NavState.home -> HomeScreen(signerDataModel = signerDataModel)
				NavState.keys -> Text("keymanager")
				NavState.scan -> Text("Brrrrr!")
				NavState.settings -> Text("Settings")
		}
*/
