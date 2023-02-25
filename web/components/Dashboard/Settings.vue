<template>
  <div>
    <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
      <section aria-labelledby="payment-details-heading">
        <span>
          <div class="shadow sm:overflow-hidden sm:rounded-md">
            <div class="bg-white py-6 px-4 sm:p-6">
              <div>
                <h2 id="payment-details-heading" class="text-lg font-medium leading-6 text-gray-900">Paramètres du compte</h2>
                <p class="mt-1 text-sm text-gray-500">Cette page vous permet de mettre à jour vos informations.</p>
              </div>

              <div class="mt-6 grid grid-cols-4 gap-6">
                <div class="col-span-4 sm:col-span-2">
                  <label for="username" class="block text-sm font-medium text-gray-700">Nom d'utilisateur</label>
                  <input v-model="username" type="text" name="username" id="username" disabled class="mt-1 block w-full rounded-md border border-gray-300 py-2 px-3 shadow-sm focus:border-gray-900 focus:outline-none focus:ring-gray-900 sm:text-sm bg-gray-100" />
                </div>

                <div class="col-span-4 sm:col-span-2">
                  <label for="email" class="block text-sm font-medium text-gray-700">Adresse mail</label>
                  <input v-model="email" type="text" name="email" id="email" disabled class="mt-1 block w-full rounded-md border border-gray-300 py-2 px-3 shadow-sm focus:border-gray-900 focus:outline-none focus:ring-gray-900 sm:text-sm bg-gray-100" />
                </div>

                <div class="col-span-4 sm:col-span-2">
                  <label for="password" class="block text-sm font-medium text-gray-700">Ancien mot de passe</label>
                  <input v-model="old_password" type="password" name="password" id="password" class="mt-1 block w-full rounded-md border border-gray-300 py-2 px-3 shadow-sm focus:border-gray-900 focus:outline-none focus:ring-gray-900 sm:text-sm" />
                </div>

                <div class="col-span-4 sm:col-span-2">
                  <label for="repassword" class="block text-sm font-medium text-gray-700">Nouveau mot de passe</label>
                  <input v-model="new_password" type="password" name="repassword" id="repassword" class="mt-1 block w-full rounded-md border border-gray-300 py-2 px-3 shadow-sm focus:border-gray-900 focus:outline-none focus:ring-gray-900 sm:text-sm" />
                </div>
              </div>
            </div>
            <div class="bg-gray-50 px-4 py-3 text-right sm:px-6">
              <button @click="updatePassword" :disabled="!old_password || !new_password" :class="'inline-flex justify-center rounded-md border border-transparent bg-green-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-green-500 focus:outline-none focus:ring-2 focus:ring-gray-900 focus:ring-offset-2' + (!old_password || !new_password ? ' opacity-50 cursor-not-allowed' : '')">
                Mettre à jour
              </button>
            </div>
            <!-- msg  -->
            <div v-if="msg" class="bg-gray-50 px-4 py-3 text-center sm:px-6">
              <div v-if="msg.type === 'success'" class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded relative" role="alert">
                <span class="block sm:inline">{{ msg.text }}</span>
              </div>
              <div v-if="msg.type === 'error'" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
                <span class="block sm:inline"><b>Erreur:</b> {{ msg.text }}</span>
              </div>
            </div>
          </div>
        </span>
      </section>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Settings',
  data() {
    return {
      username: '',
      email: '',
      old_password: '',
      new_password: '',
      msg: null,
    }
  },
  mounted() {
    this.username = this.$store.state.users.user.username;
    this.email = this.$store.state.users.user.email;
  },
  methods: {
    updatePassword() {
      this.$store.dispatch('users/updatePassword', {
        old_password: this.old_password,
        new_password: this.new_password
      }).then(() => {
        this.msg = {
          type: 'success',
          text: 'Mot de passe mis à jour avec succès'
        }
        this.old_password = '';
        this.new_password = '';
      }).catch((err) => {
        // if 401
        if (err.response.status === 401) {
          this.msg = {
            type: 'error',
            text: 'Ancien mot de passe incorrect'
          }
        } else {
          this.msg = {
            type: 'error',
            text: 'Une erreur est survenue'
          }
        }
      })
    }
  }
};
</script>