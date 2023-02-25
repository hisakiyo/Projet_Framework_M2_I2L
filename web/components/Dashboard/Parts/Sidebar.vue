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
              <component :is="item.icon" class="mr-4 h-6 w-6 flex-shrink-0 text-cyan-200" aria-hidden="true" />
              {{ item.name }}
            </NuxtLink>
          </div>
          <div class="mt-6 pt-6">
            <div class="space-y-1 px-2">
              <NuxtLink v-for="item in secondaryNavigation" :key="item.name" :to="item.href" :class="[item.current ? 'bg-cyan-800 text-white' : 'text-cyan-100 hover:text-white hover:bg-cyan-600', 'group flex items-center px-2 py-2 text-sm leading-6 font-medium rounded-md']" >
                <component :is="item.icon" class="mr-4 h-6 w-6 text-cyan-200" aria-hidden="true" />
                {{ item.name }}
              </NuxtLink>
              <span class="group flex items-center px-2 py-2 text-sm leading-6 font-medium rounded-md text-cyan-100 hover:text-white hover:bg-cyan-600" @click="logout">
                <svg class="mr-4 h-6 w-6 text-cyan-200" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H3m18 8H9a2 2 0 01-2-2V7a2 2 0 012-2h10a2 2 0 012 2v10a2 2 0 01-2 2z"></path>
                </svg>
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
        { name: 'Accueil', href: '/dashboard/', icon: '', current: this.$router.currentRoute.path === '/dashboard/' },
        { name: 'Historique', href: '/dashboard/history/', icon: '', current: this.$router.currentRoute.path === '/dashboard/history/' },
        { name: 'Balances', href: '/dashboard/balance/', icon: '', current: this.$router.currentRoute.path === '/dashboard/balance/' },
        { name: 'Achat / Revente', href: '/dashboard/buy-sales/', icon: '', current: this.$router.currentRoute.path === '/dashboard/buy-sales/' },
      ],
      secondaryNavigation: [
        { name: 'Paramètres', href: '/dashboard/settings/', icon: '', current: this.$router.currentRoute.path === '/dashboard/settings/' },
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