package io.parity.signer

import android.os.Bundle
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.MaterialTheme
import androidx.compose.ui.Modifier
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

		enableEdgeToEdge()

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



