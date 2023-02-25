import JWTDecode from 'jwt-decode'
import cookieparser from 'cookieparser'

export const actions = {
    nuxtServerInit({ commit }, { req }) {
        if (process.server && process.static) return
        if (!req.headers.cookie) return

        const parsed = cookieparser.parse(req.headers.cookie)
        const accessTokenCookie = parsed.token

        if (!accessTokenCookie) return

        let decoded = ''

        try {
            decoded = JWTDecode(accessTokenCookie)
        } catch (err) {
            return
        }

        // If token is created before 27-01-2023, destroy it. Prevents old tokens from being used.
        if (decoded.iat < 1674833088) {
            return
        }

        // If token is 1 week old, destroy it. Prevents old tokens from being used.
        if (decoded.iat < (Date.now() / 1000) - 604800) {
            return
        }

        commit('users/SET_USER', decoded)
    }
}