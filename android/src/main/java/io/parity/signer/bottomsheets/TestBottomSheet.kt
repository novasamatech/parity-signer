package io.parity.signer.bottomsheets

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextDirection.Companion.Content
import androidx.compose.ui.unit.dp
import io.parity.signer.R

/**
 * @author Dmitry Borodin on 9/27/22. todo remove
 */
@Composable
fun TestBottomSheet(action: () -> Unit) {
	Box(
		modifier = Modifier
            .height(300.dp)
            .background(Color.White)
	) {
		Column {
			Box(modifier = Modifier.align(Alignment.CenterHorizontally)) {
				Button(onClick = action) {
					Text("Tap to close")
				}
			}
		}
	}
}
