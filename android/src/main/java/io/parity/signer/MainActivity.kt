package io.parity.signer

import android.os.Bundle
import androidx.activity.compose.setContent
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.material.ExperimentalMaterialApi
import androidx.core.view.WindowCompat
import androidx.navigation.compose.rememberNavController
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.addVaultLogger
import io.parity.signer.ui.rootnavigation.RootNavigationGraph
import io.parity.signer.ui.theme.SignerNewTheme

@ExperimentalMaterialApi
@ExperimentalAnimationApi
class MainActivity : AppCompatActivity() {

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		ServiceLocator.initActivityDependencies(this)

		WindowCompat.enableEdgeToEdge(window)

		setContent {
			SignerNewTheme {
				RootNavigationGraph(
					navController = rememberNavController().apply {
						addVaultLogger(
							"Root Nav graph controller"
						)
					})
			}
		}
	}

	override fun onDestroy() {
		ServiceLocator.deinitActivityDependencies()
		super.onDestroy()
	}
}



