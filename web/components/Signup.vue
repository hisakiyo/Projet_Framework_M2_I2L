<template>
  <div class="flex h-full">
    <div class="flex flex-1 flex-col justify-center py-12 px-4 sm:px-6 lg:flex-none lg:px-20 xl:px-24">
      <div class="mx-auto w-full max-w-sm lg:w-96">
        <div>
          <h2 class="mt-6 text-3xl font-bold tracking-tight text-gray-900">Rejoignez-nous</h2>
        </div>

        <div class="mt-8">
          <div class="mt-6">
            <span class="space-y-6">
              <div>
                <label for="username" class="block text-sm font-medium text-gray-700">Nom d'utilisateur</label>
                <div class="mt-1">
                  <input v-model="username" id="username" name="username" type="text" autocomplete="text" required class="block w-full appearance-none rounded-md border border-gray-300 px-3 py-2 placeholder-gray-400 shadow-sm focus:border-sky-500 focus:outline-none focus:ring-sky-500 sm:text-sm" />
                </div>
              </div>
              <div>
                <label for="email" class="block text-sm font-medium text-gray-700">Adresse mail</label>
                <div class="mt-1">
                  <input v-model="email" id="email" name="email" type="email" autocomplete="email" required class="block w-full appearance-none rounded-md border border-gray-300 px-3 py-2 placeholder-gray-400 shadow-sm focus:border-sky-500 focus:outline-none focus:ring-sky-500 sm:text-sm" />
                </div>
              </div>

              <div class="space-y-1">
                <label for="password" class="block text-sm font-medium text-gray-700">Mot de passe</label>
                <div class="mt-1">
                  <input v-model="password" id="password" name="password" type="password" autocomplete="current-password" required class="block w-full appearance-none rounded-md border border-gray-300 px-3 py-2 placeholder-gray-400 shadow-sm focus:border-sky-500 focus:outline-none focus:ring-sky-500 sm:text-sm" />
                </div>
              </div>

              <div class="space-y-1">
                <label for="repassword" class="block text-sm font-medium text-gray-700">Répétez votre mot de passe</label>
                <div class="mt-1">
                  <input v-model="repassword" id="repassword" name="repassword" type="password" autocomplete="repassword" required class="block w-full appearance-none rounded-md border border-gray-300 px-3 py-2 placeholder-gray-400 shadow-sm focus:border-sky-500 focus:outline-none focus:ring-sky-500 sm:text-sm" />
                </div>
              </div>

              <div>
                <button @click="register" class="flex w-full justify-center rounded-md border border-transparent bg-sky-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-sky-700 focus:outline-none focus:ring-2 focus:ring-sky-500 focus:ring-offset-2">M'inscrire</button>
              </div>

              <!-- Message with an background color depending on the type -->
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
  name: 'Signup',
  data() {
    return {
      username: 'ez',
      email: 'ez@ez.ez',
      password: 'ez',
      repassword: 'ez',
      msg: null,
    };
  },
  methods: {
    async register() {
      if (this.password !== this.repassword) {
        alert('Les mots de passe ne correspondent pas');
        return;
      }
      this.$store.dispatch('users/register', {
        username: this.username,
        email: this.email,
        password: this.password,
      }).then(() => {
        this.msg = {
          type: 'success',
          text: 'Inscription réussie',
        };
      }).catch((err) => {
        // if 409, the username is already taken
        if (err.response.status === 409) {
          this.msg = {
            type: 'error',
            text: 'Ce nom d\'utilisateur est déjà pris',
          };
        } else {
          this.msg = {
            type: 'error',
            text: 'Une erreur est survenue',
          };
        }
      });
    },
  },
}
</script>