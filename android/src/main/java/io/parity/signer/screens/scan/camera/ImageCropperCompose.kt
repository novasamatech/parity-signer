package io.parity.signer.screens.scan.camera

import android.content.res.Configuration
import android.graphics.Rect
import androidx.camera.core.ImageProxy
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalConfiguration
import androidx.compose.ui.unit.dp


object ImageCropperCompose {
	@Composable
	@androidx.annotation.OptIn(androidx.camera.core.ExperimentalGetImage::class)
	fun makeCropRect(image: ImageProxy): Rect? {
		val configuration: Configuration = LocalConfiguration.current
		val screenHeight = configuration.screenHeightDp.dp
		val screenWidth = configuration.screenWidthDp.dp

		val (photoHeight, photoWIdth) = if (image.imageInfo.rotationDegrees == 90 || image.imageInfo.rotationDegrees == 270) {
			// need to rotate image
			Pair(image.image?.height, image.image?.width)
		} else {
			// rotation probably vertical - don't won't
			Pair(image.image?.width, image.image?.height)
		}
		if (photoHeight == null || photoWIdth == null) {
			return null
		}

		//todo dmitry check that forther logic if we changed rotation here




			//todo dmitry calculate what to crop taking rotation into account.
//		and move this logic into our usual cropper or even cache it.
//		val min =
//		todo remove below
		return Rect(0,0,0,0)
	}
}
