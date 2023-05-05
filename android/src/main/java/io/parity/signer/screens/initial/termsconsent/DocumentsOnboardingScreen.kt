package io.parity.signer.screens.initial.termsconsent

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.CheckboxWithText
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.textDisabled


@Composable
fun OnboardingApproveDocumentsScreen(
	onAgree: Callback,
	onTos: Callback,
	onPp: Callback,
) {
	val isAccepted = remember { mutableStateOf(false) }

	Column(Modifier.fillMaxSize(1f)) {
		Spacer(modifier = Modifier.padding(top = 24.dp))
		Text(
			text = stringResource(R.string.documents_approve_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			textAlign = TextAlign.Center,
			modifier = Modifier
				.fillMaxWidth(1f)
				.padding(horizontal = 24.dp, vertical = 16.dp)
		)

		Surface(
			shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
			color = MaterialTheme.colors.fill6,
			modifier = Modifier.padding(16.dp)
		) {
			Column(
				horizontalAlignment = Alignment.CenterHorizontally,
			) {
				Row(
					modifier = Modifier.clickable(onClick = onPp),
					verticalAlignment = Alignment.CenterVertically,
				) {
					Text(
						text = stringResource(R.string.documents_privacy_policy),
						color = MaterialTheme.colors.primary,
						style = SignerTypeface.TitleS,
						modifier = Modifier
							.padding(horizontal = 16.dp, vertical = 14.dp)
							.weight(1f)
					)
					Image(
						imageVector = Icons.Filled.ChevronRight,
						contentDescription = null,
						colorFilter = ColorFilter.tint(MaterialTheme.colors.textDisabled),
						modifier = Modifier
							.size(28.dp)
							.padding(end = 8.dp)
					)
				}
				SignerDivider()
				Row(
					modifier = Modifier.clickable(onClick = onTos),
					verticalAlignment = Alignment.CenterVertically,
				) {
					Text(
						text = stringResource(R.string.documents_terms_of_service),
						color = MaterialTheme.colors.primary,
						style = SignerTypeface.TitleS,
						modifier = Modifier
							.padding(horizontal = 16.dp, vertical = 14.dp)
							.weight(1f)
					)
					Image(
						imageVector = Icons.Filled.ChevronRight,
						contentDescription = null,
						colorFilter = ColorFilter.tint(MaterialTheme.colors.textDisabled),
						modifier = Modifier
							.size(28.dp)
							.padding(end = 8.dp)
					)
				}
			}
		}

		Spacer(modifier = Modifier.weight(1f))
		CheckboxWithText(
			checked = isAccepted.value,
			text = stringResource(R.string.documents_accepted_checkmark),
			modifier = Modifier.padding(horizontal = 24.dp, vertical = 8.dp),
		) {
			isAccepted.value = it
		}
		PrimaryButtonWide(
			label = stringResource(R.string.documents_accept_cta_button),
			modifier = Modifier.padding(24.dp),
			isEnabled = isAccepted.value,
			onClicked = onAgree,
		)
	}
}


@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewApproveDocumentsScreen() {
	SignerNewTheme {
		//doesn't work in dark mode? Check runtime, it's preview broken for this library
		OnboardingApproveDocumentsScreen({}, {}, {})
	}
}
