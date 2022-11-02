package io.parity.signer.components.qrcode

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.ui.helpers.PreviewData
import kotlinx.coroutines.delay
import kotlin.time.Duration.Companion.milliseconds

@OptIn(ExperimentalUnsignedTypes::class)
@Composable
fun <T> AnimatedQrKeysInfo(
	input: T,
	provider: AnimatedQrKeysProvider<T>,
	modifier: Modifier = Modifier
) {
	val qrRounding = dimensionResource(id = R.dimen.qrShapeCornerRadius)
	val DELAY = 125.milliseconds //FPS 8
	val qrCodes =
		remember { mutableStateOf<List<ImageBitmap>>(emptyList()) }
	val currentCode = remember { mutableStateOf<ImageBitmap?>(null) }

	Box(
		modifier = modifier
			.fillMaxWidth(1f)
			.aspectRatio(1.1f)
			.background(
				Color.White,
				RoundedCornerShape(qrRounding)
			),
		contentAlignment = Alignment.Center,
	) {
		currentCode.value?.let { currentImage ->
			Image(
				bitmap = currentImage,
				contentDescription = stringResource(R.string.qr_with_address_to_scan_description),
				contentScale = ContentScale.Fit,
				modifier = Modifier.size(264.dp)
			)
		}
	}

	LaunchedEffect(key1 = input) {
		provider.getQrCodesList(input)
			?.map { it.intoImageBitmap() }
			?.let { qrCodes.value = it }
	}

	LaunchedEffect(key1 = qrCodes.value) {
		if (qrCodes.value.isEmpty()) return@LaunchedEffect
		var index = 0
		while (true) {
			currentCode.value = qrCodes.value[index]
			if (index < qrCodes.value.lastIndex) {
				index++
			} else {
				index = 0
			}
			delay(DELAY)
		}
	}
}


interface AnimatedQrKeysProvider<T> {
	suspend fun getQrCodesList(input: T): List<List<UByte>>?
}

class EmptyAnimatedQrKeysProvider : AnimatedQrKeysProvider<Any> {
	override suspend fun getQrCodesList(input: Any): List<List<UByte>> {
		return listOf(PreviewData.exampleQRCode)
	}
}
