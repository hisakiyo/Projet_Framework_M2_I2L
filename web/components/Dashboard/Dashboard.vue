<template>
  <div>
    <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
      <h2 class="text-lg font-medium leading-6 text-gray-900">Tableau de bord</h2>
      <div class="mt-2 grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
        <div v-for="card in cards" :key="card.name" class="overflow-hidden rounded-lg bg-white shadow">
          <div class="p-5">
            <div class="flex items-center">
              <div class="flex-shrink-0">
              </div>
              <div class="ml-5 w-0 flex-1">
                <dl>
                  <dt class="truncate text-sm font-medium text-gray-500">{{ card.name }}</dt>
                  <dd>
                    <div class="text-lg font-medium text-gray-900">{{ card.amount }}</div>
                  </dd>
                </dl>
              </div>
            </div>
          </div>
          <div class="bg-gray-50 px-5 py-3">
            <div class="text-sm">
              <a :href="card.href" class="font-medium text-cyan-700 hover:text-cyan-900">Voir tout</a>
            </div>
          </div>
        </div>
      </div>
    </div>

    <h2 class="mx-auto mt-8 max-w-6xl px-4 text-lg font-medium leading-6 text-gray-900 sm:px-6 lg:px-8">Activité récente</h2>

    <div class="hidden sm:block">
      <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
        <div class="mt-2 flex flex-col">
          <div class="min-w-full overflow-hidden overflow-x-auto align-middle shadow sm:rounded-lg">
            <table class="min-w-full divide-y divide-gray-200">
              <thead>
              <tr>
                <th class="bg-gray-50 px-6 py-3 text-left text-sm font-semibold text-gray-900" scope="col">Transaction</th>
                <th class="bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900" scope="col">Montant</th>
                <th class="hidden bg-gray-50 px-6 py-3 text-left text-sm font-semibold text-gray-900 md:block" scope="col">Statut</th>
                <th class="bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900" scope="col">Date</th>
              </tr>
              </thead>
              <tbody class="divide-y divide-gray-200 bg-white">
              <tr v-for="transaction in transactions" :key="transaction.id" class="bg-white">
                <td class="w-full max-w-0 whitespace-nowrap px-6 py-4 text-sm text-gray-900">
                  <div class="flex">
                    <a :href="transaction.href" class="group inline-flex space-x-2 truncate text-sm">
                      <p class="truncate text-gray-500 group-hover:text-gray-900">{{ transaction.name }}</p>
                    </a>
                  </div>
                </td>
                <td class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <span class="font-medium text-gray-900">{{ transaction.amount }}</span>
                  {{ transaction.currency }}
                </td>
                <td class="hidden whitespace-nowrap px-6 py-4 text-sm text-gray-500 md:block">
                  <span :class="[statusStyles[transaction.status], 'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium capitalize']">{{ transaction.status }}</span>
                </td>
                <td class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <time :datetime="transaction.datetime">{{ transaction.date }}</time>
                </td>
              </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script >
export default {
  name: 'Dashboard',
  data() {
    return {
      cards: [
        { name: 'Solde du compte', href: '#', icon: '', amount: '€ 30,659.45' },
        // More items...
      ],
      transactions: [
        {
          id: 1,
          name: 'Payment to Molly Sanders',
          href: '#',
          amount: '$20,000',
          currency: 'USD',
          status: 'success',
          date: 'July 11, 2020',
          datetime: '2020-07-11',
        },
        {
          id: 2,
          name: 'Bitcoin',
          href: '#',
          amount: '$90,000',
          currency: 'USD',
          status: 'processing',
          date: 'July 11, 2020',
          datetime: '2020-07-11',
        },
        {
          id: 3,
          name: 'Payment to wip corp',
          href: '#',
          amount: '$100,000',
          currency: 'USD',
          status: 'failed',
          date: 'July 11, 2020',
          datetime: '2020-07-11',
        },
        // More transactions...
      ],
       statusStyles: {
        success: 'bg-green-100 text-green-800',
        processing: 'bg-yellow-100 text-yellow-800',
        failed: 'bg-gray-100 text-gray-800',
      }
    }
  }
};
</script>