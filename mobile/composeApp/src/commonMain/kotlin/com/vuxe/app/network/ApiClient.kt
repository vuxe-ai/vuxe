package com.vuxe.app.network

import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class LoginRequest(val token: String)

@Serializable
data class LoginResponse(
    val user_id: String,
    val external_id: String,
    val session_token: String
)

@Serializable
data class HealthResponse(
    val status: String,
    val version: String
)

/**
 * API client for the Vuxe backend.
 *
 * Configure the `baseUrl` to point at your hosted or self-hosted instance.
 */
class VuxeApi(
    private val baseUrl: String = "http://10.0.2.2:8080", // Android emulator → host
    private val client: HttpClient = HttpClient {
        install(ContentNegotiation) {
            json(Json {
                ignoreUnknownKeys = true
                isLenient = true
            })
        }
    }
) {
    /**
     * Check backend health.
     */
    suspend fun health(): HealthResponse =
        client.get("$baseUrl/health").body()

    /**
     * Authenticate with a Clerk or OIDC JWT and receive a session token.
     */
    suspend fun login(externalJwt: String): LoginResponse =
        client.post("$baseUrl/auth/login") {
            contentType(ContentType.Application.Json)
            setBody(LoginRequest(token = externalJwt))
        }.body()

    /**
     * Example protected call: ping the backend with the session token.
     */
    suspend fun whoami(sessionToken: String): String {
        // Phase 2+ will have a real /auth/whoami endpoint
        return "TODO: add /auth/whoami to backend"
    }
}
