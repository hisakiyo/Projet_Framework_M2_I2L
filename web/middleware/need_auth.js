export default function ({ store, redirect, route }) {
    // Sécurité utilisateur loggé
    if (!store.state.users.user && route.path !== '/' && route.path !== '/signup') {
        redirect('/')
    }
    // if logged and / or /signup
    if (store.state.users.user && (route.path === '/' || route.path === '/signup')) {
        redirect('/dashboard')
    }
}
