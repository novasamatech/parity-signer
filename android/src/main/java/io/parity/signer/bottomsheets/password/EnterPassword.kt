package io.parity.signer.bottomsheets.password

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.unit.dp
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.KeyCardOld
import io.parity.signer.components.SingleTextInput
import io.parity.signer.uniffi.MEnterPassword

@Composable
fun EnterPassword(
	enterPassword: MEnterPassword,
	proceed: (String) -> Unit
) {
	val password = remember {
		mutableStateOf("")
	}

	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	Column(
		horizontalAlignment = Alignment.CenterHorizontally,
		modifier = Modifier.padding(20.dp)
	) {
		HeaderBar(line1 = "SECRET PATH", line2 = "///password")
		KeyCardOld(
			identity = enterPassword.authorInfo
		)
		if (enterPassword.counter > 0u) {
			Text("Attempt " + enterPassword.counter.toString() + " of 3")
		}
		SingleTextInput(
			content = password,
			update = { password.value = it },
			onDone = {
				if (password.value.isNotBlank()) {
					proceed(password.value)
				}
			},
			prefix = { Text("///") },
			focusManager = focusManager,
			focusRequester = focusRequester
		)
		BigButton(
			text = "Next",
			isCrypto = true,
			action = {
				proceed(password.value)
			},
			isDisabled = password.value.isBlank()
		)
	}
}


/**
 * Local copy of shared [MEnterPassword] class
 */
 class EnterPasswordModel(

) {
	companion object {
//		fun createStub(): EnterPasswordModel = EnterPasswordModel(
//			keys = listOf(
//				KeyModel.createStub(),
//				KeyModel(
//					addressKey = "address key2",
//					base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9sdfsdfsdfS1repo5EYjGG",
//					identicon = PreviewData.exampleIdenticonPng,
//					hasPwd = true,
//					path = "//polkadot//path3",
//					multiselect = false,
//					secretExposed = false,
//					seedName = "sdsdsd",
//				),
//			),
//			root = KeyModel(
//				identicon = PreviewData.exampleIdenticonPng,
//				addressKey = "address key",
//				base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
//				hasPwd = true,
//				path = "//polkadot",
//				multiselect = false,
//				secretExposed = false,
//				seedName = "sdsdsd",
//			),
//			network = NetworkModel("network title", "network logo"),
//			multiselectCount = "5",
//			multiselectMode = false,
//		)
	}
}

fun MEnterPassword.toEnterPasswordModel() = EnterPasswordModel(

)
