<template>
  <div>
    <div class="hidden lg:fixed lg:inset-y-0 lg:flex lg:w-64 lg:flex-col">
      <div class="flex flex-grow flex-col overflow-y-auto bg-cyan-700 pt-5 pb-4">
        <div class="flex flex-shrink-0 items-center px-4">
          <p class="text-2xl text-white font-medium">CryptoMarket</p>
        </div>
        <nav class="mt-5 flex flex-1 flex-col divide-y divide-cyan-800 overflow-y-auto" aria-label="Sidebar">
          <div class="space-y-1 px-2">
            <NuxtLink v-for="item in navigation" :key="item.name" :to="item.href" :class="[item.current ? 'bg-cyan-800 text-white' : 'text-cyan-100 hover:text-white hover:bg-cyan-600', 'group flex items-center px-2 py-2 text-sm leading-6 font-medium rounded-md']" :aria-current="item.current ? 'page' : undefined">
              <span class="material-icons mr-2" :class="[item.current ? 'text-white' : 'text-cyan-200']">
                {{ item.icon }}
              </span>
              {{ item.name }}
            </NuxtLink>
          </div>
          <div class="mt-6 pt-6">
            <div class="space-y-1 px-2">
              <NuxtLink v-for="item in secondaryNavigation" :key="item.name" :to="item.href" :class="[item.current ? 'bg-cyan-800 text-white' : 'text-cyan-100 hover:text-white hover:bg-cyan-600', 'group flex items-center px-2 py-2 text-sm leading-6 font-medium rounded-md']" >
                <span class="material-icons mr-2" :class="[item.current ? 'text-white' : 'text-cyan-200']">
                  {{ item.icon }}
                </span>
                {{ item.name }}
              </NuxtLink>
              <span class="group flex items-center px-2 py-2 text-sm leading-6 font-medium rounded-md text-cyan-100 hover:text-white hover:bg-cyan-600" @click="logout">
                <span class="material-icons mr-2">
                  logout
                </span>
                Déconnexion
              </span>
            </div>
          </div>
        </nav>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Sidebar',
  components: {},
  data() {
    return {
       navigation: [
        { name: 'Accueil', href: '/dashboard/', icon: 'dashboard', current: this.$router.currentRoute.path === '/dashboard/' },
        { name: 'Historique', href: '/dashboard/history/', icon: 'history', current: this.$router.currentRoute.path === '/dashboard/history/' },
        { name: 'Balances', href: '/dashboard/balance/', icon: 'account_balance', current: this.$router.currentRoute.path === '/dashboard/balance/' },
        { name: 'Achat / Revente', href: '/dashboard/buy-sales/', icon: 'local_mall', current: this.$router.currentRoute.path === '/dashboard/buy-sales/' },
      ],
      secondaryNavigation: [
        { name: 'Paramètres', href: '/dashboard/settings/', icon: 'settings', current: this.$router.currentRoute.path === '/dashboard/settings/' },
      ],
      sidebarOpen: false,
    }
  },
  methods: {
    logout() {
      this.$store.dispatch('users/logout')
      .then(() => {
        this.$router.push('/')
      })
      .catch((err) => {
        console.log(err)
      })
    }
  }
};
</script>