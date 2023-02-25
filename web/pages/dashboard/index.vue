<template>
  <div class="min-h-full bg-gray-100">
    <Sidebar />

    <div class="flex flex-1 flex-col lg:pl-64">
      <main class="flex-1 pb-8">
        <PageHead />

        <div class="mt-8">
          <Dashboard />
        </div>
      </main>
    </div>
  </div>
</template>

<script>
import Sidebar from '~/components/Dashboard/Parts/Sidebar.vue'
import PageHead from '~/components/Dashboard/Parts/PageHead.vue'
import Dashboard from '~/components/Dashboard/Dashboard.vue'

export default {
  name: 'DashboardComplete',
  components: {
    Sidebar,
    PageHead,
    Dashboard,
  },
};
</script>