import JWTDecode from 'jwt-decode'
export const strict = false

export const state = () => ({
    user: null,
})

export const mutations = {
    SET_USER(state, user) {
        state.user = user
    },
}

export const actions = {
    async login({ commit }, account) {
        const res =  await this.$axios.$post('/api/login', account)
        const user = JWTDecode(res.token)
        commit('SET_USER', user)
    },
    async logout({ commit }) {
        await this.$axios.$post('/api/logout')
        commit('SET_USER', null)
    },
    async register({ commit }, account) {
        const res = await this.$axios.$post('/api/register', account)
        const user = JWTDecode(res.token)
        commit('SET_USER', user)
    }
}