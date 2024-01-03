package io.parity.signer.screens.initial.eachstartchecks.osversion

import android.content.res.Configuration
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.SecurityUpdate
import androidx.compose.material.icons.outlined.SettingsBackupRestore
import androidx.compose.material.icons.outlined.UpdateDisabled
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.ExperimentalTextApi
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.buildAnnotatedString
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.withAnnotation
import androidx.compose.ui.text.withStyle
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.domain.Callback
import io.parity.signer.screens.scan.errors.COMPOSE_URL_TAG_ANNOTATION
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill12
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.pink500
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.ui.theme.textTertiary

@Composable
fun WrongOsVersionNotificationScreen(
	currentOSVersion: String,
	minimalOsVersion: String,
	onProceed: Callback
) {
	Column(
		modifier = Modifier
			.padding(24.dp)
	) {
		Spacer(Modifier.weight(0.5f))
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState())
		) {
			Image(
				imageVector = Icons.Outlined.SecurityUpdate,
				contentDescription = null,
				colorFilter = ColorFilter.tint(MaterialTheme.colors.pink500),
				modifier = Modifier
					.padding(horizontal = 8.dp)
					.size(80.dp)
					.align(Alignment.CenterHorizontally)
			)
			Spacer(modifier = Modifier.padding(top = 16.dp))
			Text(
				modifier = Modifier
					.fillMaxWidth(1f),
				text = stringResource(R.string.initial_screen_outdated_os_title),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleL,
				textAlign = TextAlign.Center,
			)
			Spacer(modifier = Modifier.padding(top = 16.dp))
			Text(
				modifier = Modifier
					.fillMaxWidth(1f),
				text = stringResource(
					R.string.initial_screen_outdated_os_description,
					currentOSVersion,
					minimalOsVersion,
				),
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.BodyL,
				textAlign = TextAlign.Center,
			)
			Spacer(modifier = Modifier.padding(top = 24.dp))

			Spacer(modifier = Modifier.padding(top = 8.dp))
			PrimaryButtonWide(
				modifier = Modifier.padding(vertical = 24.dp),
				label = stringResource(R.string.onboarding_skip),
				onClicked = onProceed,
			)
		}
		Spacer(Modifier.weight(0.5f))
	}
}


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun WrongOsVersionNotificationScreenPreview() {
	Box(modifier = Modifier.fillMaxSize()) {
		SignerNewTheme() {
			WrongOsVersionNotificationScreen(
				"6.0", "7.1",
				onProceed = {},
			)
		}
	}
}


