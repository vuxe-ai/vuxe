package com.vuxe.app

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

@Composable
fun App() {
    MaterialTheme {
        var showContent by remember { mutableStateOf(false) }
        Column(
            modifier = Modifier.fillMaxSize(),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center
        ) {
            Text(
                text = "Vuxe",
                style = MaterialTheme.typography.headlineLarge
            )
            Spacer(modifier = Modifier.height(16.dp))
            Text("Your daily audio briefing, coming soon.")
            Spacer(modifier = Modifier.height(24.dp))
            Button(onClick = { showContent = !showContent }) {
                Text(if (showContent) "Hide" else "Show more")
            }
            if (showContent) {
                Spacer(modifier = Modifier.height(16.dp))
                Text(
                    text = "Phase 1 — Foundation\n\n• Rust + Axum backend\n• Postgres database\n• Auth via Clerk or OIDC\n• Docker Compose self-hosting",
                    style = MaterialTheme.typography.bodyMedium
                )
            }
        }
    }
}
