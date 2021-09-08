package io.parity.signer

import android.content.Context
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.appcompat.app.AppCompatActivity
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.Lock
import androidx.compose.material.icons.filled.Menu
import androidx.compose.material.icons.filled.Settings
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.Observer
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.google.common.util.concurrent.ListenableFuture
import io.parity.signer.components.BottomBar
import io.parity.signer.modals.WaitingScreen
import io.parity.signer.models.SignerDataModel
import io.parity.signer.screens.HomeScreen
import io.parity.signer.screens.KeyManager
import io.parity.signer.screens.SettingsScreen
import io.parity.signer.ui.theme.ParitySignerTheme
import java.util.concurrent.CompletableFuture
import android.Manifest
import android.content.pm.PackageManager
import android.widget.Toast
import androidx.core.app.ActivityCompat

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
		val navController = rememberNavController()
		var onBoardingDone = signerDataModel.onBoardingDone.observeAsState()

		when (onBoardingDone.value) {
			OnBoardingState.Yes -> {
				//Structure to contain all app
				Scaffold(
					topBar = {
						TopAppBar(
							title = { Text("Parity Signer") },
							navigationIcon = {
								IconButton(onClick = {
									signerDataModel.totalRefresh()
									navController.navigateUp()
								}) {
									Icon(Icons.Default.ArrowBack, contentDescription = "go back")
								}
							}
						)
					},
					bottomBar = {
						BottomAppBar {
							IconButton(onClick = {
								signerDataModel.totalRefresh()
								navController.navigate(SignerScreen.Keys.name)
							}) {
								Icon(Icons.Default.Lock, contentDescription = "Key manager")
							}
							Spacer(Modifier.weight(1f, true))
							IconButton(onClick = {
								signerDataModel.totalRefresh()
								navController.navigate(SignerScreen.Settings.name)
							}) {
								Icon(Icons.Default.Settings, contentDescription = "Settings")
							}
						}
					},
					floatingActionButton = {
						FloatingActionButton(onClick = {
							signerDataModel.totalRefresh()
							navController.navigateUp()
						}) {
							Icon(
								painter = painterResource(id = R.drawable.icon),
								modifier = Modifier.size(60.dp),
								contentDescription = "Home"
							)
						}
					},
					floatingActionButtonPosition = FabPosition.Center,
					isFloatingActionButtonDocked = true
				) { innerPadding ->
					NavHost(
						navController = navController,
						SignerScreen.Home.name,
						modifier = Modifier.padding(innerPadding)
					) {
						composable(SignerScreen.Home.name) {
							HomeScreen(
								signerDataModel = signerDataModel)
						}
						composable(SignerScreen.Keys.name) {
							KeyManager(signerDataModel = signerDataModel)
						}
						composable(SignerScreen.Settings.name) {
							SettingsScreen(signerDataModel = signerDataModel)
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
