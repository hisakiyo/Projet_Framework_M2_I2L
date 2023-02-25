<template>
  <div>
    <div class="mx-auto max-w-6xl px-4 sm:px-6 lg:px-8">
      <section aria-labelledby="payment-details-heading">
        <form action="#" method="POST">
          <div class="shadow sm:overflow-hidden sm:rounded-md">
            <div class="bg-white py-6 px-4 sm:p-6">
              <div>
                <h2 id="payment-details-heading" class="text-lg font-medium leading-6 text-gray-900">Paramètres du compte</h2>
                <p class="mt-1 text-sm text-gray-500">Cette page vous permet de mettre à jour vos informations.</p>
              </div>

              <div class="mt-6 grid grid-cols-4 gap-6">
                <div class="col-span-4 sm:col-span-2">
                  <label for="username" class="block text-sm font-medium text-gray-700">Nom d'utilisateur</label>
                  <input type="text" name="username" id="username" class="mt-1 block w-full rounded-md border border-gray-300 py-2 px-3 shadow-sm focus:border-gray-900 focus:outline-none focus:ring-gray-900 sm:text-sm" />
                </div>

                <div class="col-span-4 sm:col-span-2">
                  <label for="email" class="block text-sm font-medium text-gray-700">Adresse mail</label>
                  <input type="text" name="email" id="email" class="mt-1 block w-full rounded-md border border-gray-300 py-2 px-3 shadow-sm focus:border-gray-900 focus:outline-none focus:ring-gray-900 sm:text-sm" />
                </div>

                <div class="col-span-4 sm:col-span-2">
                  <label for="password" class="block text-sm font-medium text-gray-700">Mot de passe</label>
                  <input type="password" name="password" id="password" class="mt-1 block w-full rounded-md border border-gray-300 py-2 px-3 shadow-sm focus:border-gray-900 focus:outline-none focus:ring-gray-900 sm:text-sm" />
                </div>

                <div class="col-span-4 sm:col-span-2">
                  <label for="repassword" class="block text-sm font-medium text-gray-700">Répétez le mot de passe</label>
                  <input type="password" name="repassword" id="repassword" class="mt-1 block w-full rounded-md border border-gray-300 py-2 px-3 shadow-sm focus:border-gray-900 focus:outline-none focus:ring-gray-900 sm:text-sm" />
                </div>
              </div>
            </div>
            <div class="bg-gray-50 px-4 py-3 text-right sm:px-6">
              <button type="submit" class="inline-flex justify-center rounded-md border border-transparent bg-green-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-green-500 focus:outline-none focus:ring-2 focus:ring-gray-900 focus:ring-offset-2">Mettre à jour</button>
            </div>
          </div>
        </form>
      </section>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Settings',
};
</script>