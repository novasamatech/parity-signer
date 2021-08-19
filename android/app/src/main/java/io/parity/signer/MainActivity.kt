package io.parity.signer

import android.os.Bundle
import android.widget.ImageButton
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.ParitySignerTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            ParitySignerTheme {
                // A surface container using the 'background' color from the theme
                Surface(color = MaterialTheme.colors.background) {
                    MainScreen()
                }
            }
        }
    }
}

/**
 * Starting screen that only lets you navigate to one of 3 usable screens
 */
@Composable
fun MainScreen() {
	Column (
		verticalArrangement = Arrangement.SpaceBetween,
		horizontalAlignment = Alignment.CenterHorizontally,
		modifier = Modifier.fillMaxSize()
	) {
		Image(
			painter = painterResource(id = R.drawable.icon),
			contentDescription = "AppIcon",
			modifier = Modifier.align(Alignment.CenterHorizontally
			)
		)
		Row(
			horizontalArrangement = Arrangement.SpaceEvenly,
			modifier = Modifier.fillMaxWidth()
		) {
			Button(
				colors = ButtonDefaults.buttonColors(
					backgroundColor = MaterialTheme.colors.background,
					contentColor = MaterialTheme.colors.onBackground,
				),
				onClick = { /*TODO*/ }
			) {
				Text(text = "Key manager")
			}
			Button(
				colors = ButtonDefaults.buttonColors(
					backgroundColor = MaterialTheme.colors.background,
					contentColor = MaterialTheme.colors.onBackground
				),
				onClick = { /*TODO*/ }
			) {
				Text(text = "Settings")
			}
		}
	}
}

@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
    ParitySignerTheme {
        MainScreen()
    }
}
