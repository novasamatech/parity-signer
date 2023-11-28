package io.parity.signer.screens.scan.camera

import android.graphics.Rect
import androidx.camera.core.ImageProxy
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalConfiguration
import androidx.compose.ui.unit.dp


object ImageCropperCompose {
	@Composable
	fun makeCropRect(image: ImageProxy): Rect {
		val configuration = LocalConfiguration.current
		val screenHeight = configuration.screenHeightDp.dp
		val screenWidth = configuration.screenWidthDp.dp

			//todo dmitry calculate what to crop taking rotation into account.
//		and move this logic into our usual cropper or even cache it.
		val min =
	}
}
