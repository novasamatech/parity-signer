package io.parity.signer

import android.content.Context
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.components.BottomBar
import io.parity.signer.components.TopBar
import io.parity.signer.models.SignerDataModel
import io.parity.signer.screens.HomeScreen
import io.parity.signer.screens.KeyManager
import io.parity.signer.screens.SettingsScreen
import io.parity.signer.screens.TransactionScreen
import io.parity.signer.ui.theme.ParitySignerTheme

class MainActivity : ComponentActivity() {

	private val signerDataModel by viewModels<SignerDataModel>()

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		signerDataModel.context = getContext()
		setContent {
			SignerApp(signerDataModel)
		}
	}

	fun getContext(): Context {
		return applicationContext
	}
}

/**
 * Main app component - hosts navhost, Rust-based source of truth, etc.
 */
@Composable
fun SignerApp(signerDataModel: SignerDataModel) {
	ParitySignerTheme {
		val allScreens = SignerScreen.values().toList()
		val navController = rememberNavController()

		//Structure to contain all app
		Scaffold(
			topBar = {
				TopBar(SignerScreen.Home, navBack = { navController.navigateUp() })
			},
			bottomBar = {
				BottomBar(
					signerDataModel = signerDataModel,
					navToKeys = { navController.navigate(SignerScreen.Keys.name) },
					navToSettings = { navController.navigate(SignerScreen.Settings.name) })
			}
		) { innerPadding ->
			NavHost(navController = navController, SignerScreen.Home.name, modifier = Modifier.padding(innerPadding)) {
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
					SettingsScreen()
				}
				composable(SignerScreen.Transaction.name) {
					TransactionScreen()
				}
			}
		}

		/*
		Surface(color = MaterialTheme.colors.background) {
			MainScreen(signerDataModel)
		}*/
	}
}

/**
 * TODO: remove junk
 */
@Composable
fun MainScreen(signerDataModel: SignerDataModel) {

}

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
