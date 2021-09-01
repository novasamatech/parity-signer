package io.parity.signer

import android.content.Context
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.appcompat.app.AppCompatActivity
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
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.Observer
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.components.BottomBar
import io.parity.signer.models.SignerDataModel
import io.parity.signer.screens.HomeScreen
import io.parity.signer.screens.KeyManager
import io.parity.signer.screens.SettingsScreen
import io.parity.signer.screens.TransactionScreen
import io.parity.signer.ui.theme.ParitySignerTheme

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
@Composable
fun SignerApp(signerDataModel: SignerDataModel) {
	ParitySignerTheme {
		//val allScreens = SignerScreen.values().toList()
		val navController = rememberNavController()
		var onBoardingDone = signerDataModel.onBoardingDone.observeAsState()

		if (onBoardingDone.value as Boolean) {
			//Structure to contain all app
			Scaffold(
				topBar = {
					TopAppBar(
						title = { Text("Parity Signer") },
						navigationIcon = {
							IconButton(onClick = { navController.navigateUp() }) {
								Icon(Icons.Default.ArrowBack, contentDescription = "go back")
							}
						}
					)
				},
				bottomBar = {
					BottomAppBar {
						IconButton(onClick = { navController.navigate(SignerScreen.Keys.name) }) {
							Icon(Icons.Default.Lock, contentDescription = "Key manager")
						}
						Spacer(Modifier.weight(1f, true))
						IconButton(onClick = { navController.navigate(SignerScreen.Settings.name) }) {
							Icon(Icons.Default.Settings, contentDescription = "Settings")
						}
					}
				},
				floatingActionButton = {
					FloatingActionButton(onClick = {
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
							signerDataModel = signerDataModel,
							navToTransaction = { navController.navigate(SignerScreen.Transaction.name) }
						)
					}
					composable(SignerScreen.Keys.name) {
						KeyManager(signerDataModel = signerDataModel)
					}
					composable(SignerScreen.Settings.name) {
						SettingsScreen(signerDataModel = signerDataModel)
					}
					composable(SignerScreen.Transaction.name) {
						TransactionScreen(signerDataModel = signerDataModel)
					}
				}
			}
		} else {
			AlertDialog(
				onDismissRequest = { /*TODO: nothing*/ },
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
