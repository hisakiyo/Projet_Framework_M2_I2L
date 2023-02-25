<template>
  <div class="bg-white shadow">
    <div class="px-4 sm:px-6 lg:mx-auto lg:max-w-6xl lg:px-8">
      <div class="py-6 md:flex md:items-center md:justify-between lg:border-t lg:border-gray-200">
        <div class="min-w-0 flex-1">
          <div class="flex items-center">
            <img class="hidden h-16 w-16 rounded-full sm:block object-cover" src="/businessman.jpeg" alt="" />
            <div>
              <div class="flex items-center">
                <img class="h-16 w-16 rounded-full sm:hidden object-cover" src="/businessman.jpeg" alt="" />
                <h1 class="ml-3 text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:leading-9">Bienvenue, Sotramp</h1>
              </div>
              <dl class="mt-6 flex flex-col sm:ml-3 sm:mt-1 sm:flex-row sm:flex-wrap">
                <dt class="sr-only">Statut du compte</dt>
                <dd class="mt-3 flex items-center text-sm font-medium capitalize text-gray-500 sm:mr-6 sm:mt-0">
                  Compte vérifié
                </dd>
              </dl>
            </div>
          </div>
        </div>
        <div class="mt-6 flex space-x-3 md:mt-0 md:ml-4">
          <button type="button" class="inline-flex items-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-cyan-500 focus:ring-offset-2">Vendre de la crypto</button>
          <button type="button" class="inline-flex items-center rounded-md border border-transparent bg-cyan-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-cyan-700 focus:outline-none focus:ring-2 focus:ring-cyan-500 focus:ring-offset-2">Acheter de la crypto</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'PageHead',
};
</script>