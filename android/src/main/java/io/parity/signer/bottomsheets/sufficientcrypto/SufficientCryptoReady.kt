package io.parity.signer.bottomsheets

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import io.parity.signer.bottomsheets.sufficientcrypto.SufficientCryptoReadyViewModel
import io.parity.signer.components.*
import io.parity.signer.domain.intoImageBitmap
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.MSufficientCryptoReady
import io.parity.signer.uniffi.MscContent

@Composable
fun SufficientCryptoReady(
	sufficientCrypto: MSufficientCryptoReady,
) {
	Surface(
		shape = MaterialTheme.shapes.modal,
		color = MaterialTheme.colors.Bg000
	) {
		Column(
			modifier = Modifier
				.fillMaxSize()
				.padding(20.dp)
		) {
			HeaderBar("Your signature", "Scan this into your application")
			Image(
				bitmap = SufficientCryptoReadyViewModel.getQrCodeBitmapFromQrCodeData(
					sufficientCrypto.sufficient
				)?.intoImageBitmap() ?: ImageBitmap(1, 1),
				contentDescription = "Signed update",
				contentScale = ContentScale.FillWidth,
				modifier = Modifier.fillMaxWidth()
			)
			KeyCardOld(
				identity = sufficientCrypto.authorInfo,
			)
			when (val c = sufficientCrypto.content) {
				is MscContent.AddSpecs -> Column {
					Text("Specs")
					NetworkCard(NetworkCardModel(c.f))
				}
				is MscContent.LoadMetadata -> Text("Metadata for " + c.name + " with version " + c.version)
				is MscContent.LoadTypes -> Column {
					Text("types " + c.types)
					IdentIcon(identicon = c.pic.toImageContent())
				}
			}
		}
	}
}
