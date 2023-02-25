<template>
    <div>
        <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
            <h2 class="text-lg font-medium leading-6 text-gray-900">Achat / Vente</h2>
        </div>
        <div class="hidden sm:block">
      <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
        <div class="mt-2 flex flex-col">
          <div class="min-w-full overflow-hidden overflow-x-auto align-middle shadow sm:rounded-lg">
            <table class="min-w-full divide-y divide-gray-200">
              <thead>
              <tr>
                <th class="bg-gray-50 px-6 py-3 text-left text-sm font-semibold text-gray-900" scope="col">Symbole</th>
                <th class="bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900" scope="col">Nom</th>
                <th class="bg-gray-50 px-6 py-3 text-left text-sm font-semibold text-gray-900" scope="col">Prix</th>
                <th class="bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900" scope="col">Date de dernière mise à jour</th>
                <th class="bg-gray-50 px-6 py-3 text-right text-sm font-semibold text-gray-900" scope="col">Actions</th>
              </tr>
              </thead>
              <tbody class="divide-y divide-gray-200 bg-white">
              <tr v-for="price in prices" :key="price" class="bg-white">
                <td class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <p class="text-gray-900 font-medium">{{ price.symbol }}</p>
                </td>
                <td class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <p class="text-gray-900 font-medium">{{ price.name }}</p>
                </td>
                <td class="whitespace-nowrap px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  <p class="text-gray-900 font-medium">{{ formattedPrice(price.price) }}</p>
                </td>

                <td class="whitespace-nowrap px-6 py-4 text-right text-sm text-gray-500">
                  <time :datetime="price.last_update">{{ formatAgo(price.last_update) }}</time>
                </td>
                <!-- One bouton "acheter" et un bouton "vendre". Avec un background et des couleurs différentes et des bords arrondis. -->
                <td class="whitespace-nowrap px-6 py-4 text-right text-sm font-medium">
                  <a href="#" class="text-indigo-600 hover:text-indigo-900">Acheter</a>
                  /
                  <a href="#" class="text-indigo-600 hover:text-indigo-900">Vendre</a>
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

<script>
export default {
    name: 'BuySales',
    data() {
      return {
        prices: [],
      }
    },
    mounted() {
      this.getPrices();
    },
    methods: {
      formatAgo(date) {
        return new Date(date).toLocaleDateString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' });
      },
      formattedPrice(price) {
        return new Intl.NumberFormat('fr-FR', { style: 'currency', currency: 'USD' }).format(price);
      },
      getPrices() {
        this.$axios.get('/api/currencies_with_price')
          .then(response => {
            this.prices = response.data;
          })
          .catch(error => {
            console.log(error);
          });
      },
    },
}
</script>