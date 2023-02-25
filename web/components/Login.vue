<template>
  <div class="flex h-full">
    <div class="flex flex-1 flex-col justify-center py-12 px-4 sm:px-6 lg:flex-none lg:px-20 xl:px-24">
      <div class="mx-auto w-full max-w-sm lg:w-96">
        <div>
          <h2 class="mt-6 text-3xl font-bold tracking-tight text-gray-900">Connectez-vous</h2>
          {{ $store.state.users.user }}
          <p class="mt-2 text-sm text-gray-600">
            <NuxtLink to="/signup" class="font-medium text-sky-600 hover:text-sky-500">Vous n'avez pas de compte ? Inscrivez-vous</NuxtLink>
          </p>
        </div>

        <div class="mt-8">
          <div class="mt-6">
            <span class="space-y-6">
              <div>
                <label for="email" class="block text-sm font-medium text-gray-700">Adresse mail</label>
                <div class="mt-1">
                  <input v-model="email" id="email" name="email" type="email" autocomplete="email" required="" class="block w-full appearance-none rounded-md border border-gray-300 px-3 py-2 placeholder-gray-400 shadow-sm focus:border-sky-500 focus:outline-none focus:ring-sky-500 sm:text-sm" />
                </div>
              </div>

              <div class="space-y-1">
                <label for="password" class="block text-sm font-medium text-gray-700">Mot de passe</label>
                <div class="mt-1">
                  <input v-model="password" id="password" name="password" type="password" autocomplete="current-password" required="" class="block w-full appearance-none rounded-md border border-gray-300 px-3 py-2 placeholder-gray-400 shadow-sm focus:border-sky-500 focus:outline-none focus:ring-sky-500 sm:text-sm" />
                </div>
              </div>

              <div class="flex items-center justify-between">
                <div class="flex items-center">
                  <input id="remember-me" name="remember-me" type="checkbox" class="h-4 w-4 rounded border-gray-300 text-sky-600 focus:ring-sky-500" />
                  <label for="remember-me" class="ml-2 block text-sm text-gray-900">Se souvenir de moi</label>
                </div>
              </div>

              <div>
                <button @click="login" class="flex w-full justify-center rounded-md border border-transparent bg-sky-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-sky-700 focus:outline-none focus:ring-2 focus:ring-sky-500 focus:ring-offset-2">Connexion {{ $store.state.users.user}}</button>
              </div>

              <div v-if="msg" :class="`bg-${msg.type}-100 border-l-4 border-${msg.type}-500 text-${msg.type}-700 p-4`">
                <p class="font-bold">{{ msg.text }}</p>
              </div>
            </span>
          </div>
        </div>
      </div>
    </div>
    <div class="relative hidden w-0 flex-1 lg:block">
      <img class="absolute inset-0 h-full w-full object-cover" src="hp_illustration.jpeg" alt="" />
    </div>
  </div>
</template>

<script>
export default {
  name: 'Login',
  data() {
    return {
      email: 'ez@ez.ez',
      password: 'ez',
      msg: null
    }
  },
  methods: {
    login() {
      this.$store.dispatch('users/login', { email: this.email, password: this.password })
      .then(() => {
        this.$router.push('/dashboard')
      })
      .catch((err) => {
        this.msg = {
          type: 'error',
          text: 'Adresse mail ou mot de passe incorrect'
        }
      })
    }
  }
}
</script>